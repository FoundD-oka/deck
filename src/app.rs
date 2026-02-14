use crate::config::AppConfig;
use crate::log_store::LogStore;
use crate::needs_input::NeedsInputDetector;
use crate::persistence;
use crate::pty_manager::PtyHandle;
use crate::session::{Session, SessionStatus};
use crate::ui;

use ftui_core::event::{Event, KeyCode, KeyEvent, KeyEventKind, Modifiers};
use ftui_core::geometry::Rect;
use ftui_layout::{Constraint, Flex};
use ftui_render::frame::Frame;
use ftui_runtime::subscription::Every;
use ftui_runtime::{Cmd, Model};

use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Panel {
    SessionList,
    DirTree,
    FilePreview,
    Log,
    Input,
}

impl Panel {
    fn next(self) -> Self {
        match self {
            Self::SessionList => Self::DirTree,
            Self::DirTree => Self::FilePreview,
            Self::FilePreview => Self::Log,
            Self::Log => Self::Input,
            Self::Input => Self::SessionList,
        }
    }

    fn prev(self) -> Self {
        match self {
            Self::SessionList => Self::Input,
            Self::DirTree => Self::SessionList,
            Self::FilePreview => Self::DirTree,
            Self::Log => Self::FilePreview,
            Self::Input => Self::Log,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogMode {
    Individual,
    Unified,
}

pub enum Msg {
    Key(KeyEvent),
    PtyPollTick,
    Noop,
}

impl From<Event> for Msg {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(k) => Msg::Key(k),
            _ => Msg::Noop,
        }
    }
}

pub struct AppState {
    pub sessions: Vec<Session>,
    pub active_session: usize,
    pub active_panel: Panel,
    pub input_text: String,
    pub config: AppConfig,
    pub log_store: LogStore,
    pub log_mode: LogMode,
    // PTY handles (runtime-only, not serialized)
    pty_handles: HashMap<Uuid, PtyHandle>,
    // needs_input detection
    needs_input_detector: NeedsInputDetector,
    last_output_at: HashMap<Uuid, Instant>,
    // Session creation dialog state
    creating_session: bool,
    create_step: CreateStep,
    create_name: String,
    create_path: String,
    // Rename dialog state
    renaming: bool,
    rename_text: String,
}

#[derive(PartialEq)]
enum CreateStep {
    Name,
    Path,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let sessions = persistence::load_sessions(&config.sessions_file_path);
        Self {
            sessions,
            active_session: 0,
            active_panel: Panel::SessionList,
            input_text: String::new(),
            config,
            log_store: LogStore::new(),
            log_mode: LogMode::Individual,
            pty_handles: HashMap::new(),
            needs_input_detector: NeedsInputDetector::new(),
            last_output_at: HashMap::new(),
            creating_session: false,
            create_step: CreateStep::Name,
            create_name: String::new(),
            create_path: String::new(),
            renaming: false,
            rename_text: String::new(),
        }
    }

    fn save(&self) {
        let _ = persistence::save_sessions(&self.config.sessions_file_path, &self.sessions);
    }

    fn handle_key(&mut self, key: KeyEvent) -> Cmd<Msg> {
        // Only process Press events
        if key.kind != KeyEventKind::Press {
            return Cmd::None;
        }

        // Session creation dialog
        if self.creating_session {
            return self.handle_create_dialog(key);
        }

        // Rename dialog
        if self.renaming {
            return self.handle_rename_dialog(key);
        }

        // Global keys
        match (key.code, key.modifiers) {
            (KeyCode::Char('c'), m) if m.contains(Modifiers::CTRL) => {
                // Send SIGINT to active session's PTY if running, otherwise quit
                if let Some(session) = self.sessions.get(self.active_session) {
                    if session.status == SessionStatus::Running
                        || session.status == SessionStatus::NeedsInput
                    {
                        if let Some(handle) = self.pty_handles.get_mut(&session.id) {
                            let _ = handle.send_sigint();
                            return Cmd::None;
                        }
                    }
                }
                return Cmd::Quit;
            }
            (KeyCode::Char('q'), _) if self.active_panel != Panel::Input => {
                self.save();
                return Cmd::Quit;
            }
            // Panel navigation: Ctrl+h/j/k/l
            (KeyCode::Char('h'), m) if m.contains(Modifiers::CTRL) => {
                self.active_panel = self.active_panel.prev();
                return Cmd::None;
            }
            (KeyCode::Char('l'), m) if m.contains(Modifiers::CTRL) => {
                self.active_panel = self.active_panel.next();
                return Cmd::None;
            }
            (KeyCode::Char('j'), m) if m.contains(Modifiers::CTRL) => {
                self.active_panel = self.active_panel.next();
                return Cmd::None;
            }
            (KeyCode::Char('k'), m) if m.contains(Modifiers::CTRL) => {
                self.active_panel = self.active_panel.prev();
                return Cmd::None;
            }
            (KeyCode::Tab, _) => {
                self.active_panel = self.active_panel.next();
                return Cmd::None;
            }
            _ => {}
        }

        // Panel-specific keys
        match self.active_panel {
            Panel::SessionList => self.handle_session_list_key(key),
            Panel::Log => self.handle_log_key(key),
            Panel::Input => self.handle_input_key(key),
            _ => Cmd::None,
        }
    }

    fn handle_session_list_key(&mut self, key: KeyEvent) -> Cmd<Msg> {
        match key.code {
            KeyCode::Up => {
                if self.active_session > 0 {
                    self.active_session -= 1;
                }
            }
            KeyCode::Down => {
                if self.active_session + 1 < self.sessions.len() {
                    self.active_session += 1;
                }
            }
            KeyCode::Char('n') => {
                self.creating_session = true;
                self.create_step = CreateStep::Name;
                self.create_name.clear();
                self.create_path.clear();
            }
            KeyCode::Char('d') => {
                if !self.sessions.is_empty() {
                    let session_id = self.sessions[self.active_session].id;
                    self.pty_handles.remove(&session_id);
                    self.last_output_at.remove(&session_id);
                    self.sessions.remove(self.active_session);
                    if self.active_session >= self.sessions.len() && self.active_session > 0 {
                        self.active_session -= 1;
                    }
                    self.save();
                }
            }
            KeyCode::Char('r') => {
                if let Some(session) = self.sessions.get(self.active_session) {
                    self.renaming = true;
                    self.rename_text = session.name.clone();
                }
            }
            KeyCode::Char('m') => {
                // Manual NeedsInput toggle
                if let Some(session) = self.sessions.get_mut(self.active_session) {
                    match session.status {
                        SessionStatus::Running => {
                            let _ = session.transition_to(SessionStatus::NeedsInput);
                            self.save();
                        }
                        SessionStatus::NeedsInput => {
                            let _ = session.transition_to(SessionStatus::Running);
                            // Reset timeout timer on manual resume
                            self.last_output_at.insert(session.id, Instant::now());
                            self.save();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Cmd::None
    }

    fn handle_log_key(&mut self, key: KeyEvent) -> Cmd<Msg> {
        match key.code {
            KeyCode::Char('t') => {
                self.log_mode = match self.log_mode {
                    LogMode::Individual => LogMode::Unified,
                    LogMode::Unified => LogMode::Individual,
                };
            }
            _ => {}
        }
        Cmd::None
    }

    fn handle_input_key(&mut self, key: KeyEvent) -> Cmd<Msg> {
        match key.code {
            KeyCode::Char(c) => {
                self.input_text.push(c);
            }
            KeyCode::Backspace => {
                self.input_text.pop();
            }
            KeyCode::Enter => {
                if !self.input_text.is_empty() && !self.sessions.is_empty() {
                    let session_idx = self.active_session;
                    let session_id = self.sessions[session_idx].id;
                    let input = self.input_text.clone();
                    self.input_text.clear();

                    // Spawn PTY if not running
                    if !self.pty_handles.contains_key(&session_id) {
                        let root_path = self.sessions[session_idx].root_path.clone();
                        match PtyHandle::spawn(&root_path, 80, 24) {
                            Ok(handle) => {
                                let pid = handle.process_id();
                                self.pty_handles.insert(session_id, handle);
                                self.last_output_at.insert(session_id, Instant::now());
                                let session = &mut self.sessions[session_idx];
                                let _ = session.transition_to(SessionStatus::Running);
                                session.pty_pid = pid;
                                session.instruction = Some(input.clone());
                                let _ = persistence::write_log_header(
                                    &session.log_path,
                                    session,
                                );
                                self.save();
                            }
                            Err(e) => {
                                self.log_store.append(
                                    session_id,
                                    format!("Failed to spawn PTY: {}\n", e).as_bytes(),
                                );
                                let session = &mut self.sessions[session_idx];
                                let _ = session.transition_to(SessionStatus::Running);
                                let _ = session.transition_to(SessionStatus::Failed);
                                self.save();
                                return Cmd::None;
                            }
                        }
                    }

                    // If session was NeedsInput, transition back to Running
                    if self.sessions[session_idx].status == SessionStatus::NeedsInput {
                        let _ = self.sessions[session_idx].transition_to(SessionStatus::Running);
                        self.last_output_at.insert(session_id, Instant::now());
                        self.save();
                    }

                    // Send input to PTY
                    if let Some(handle) = self.pty_handles.get_mut(&session_id) {
                        let _ = handle.send_input(&format!("{}\n", input));
                    }
                }
            }
            KeyCode::Escape => {
                self.active_panel = Panel::SessionList;
            }
            _ => {}
        }
        Cmd::None
    }

    fn handle_pty_poll(&mut self) -> Cmd<Msg> {
        // Collect info about active sessions (Running + NeedsInput) to avoid borrow conflicts
        let active: Vec<(Uuid, std::path::PathBuf, SessionStatus)> = self
            .sessions
            .iter()
            .filter(|s| {
                s.status == SessionStatus::Running || s.status == SessionStatus::NeedsInput
            })
            .map(|s| (s.id, s.log_path.clone(), s.status.clone()))
            .collect();

        let mut finished: Vec<(Uuid, bool, u32)> = Vec::new();
        let mut needs_input_detected: Vec<Uuid> = Vec::new();

        for (session_id, log_path, status) in &active {
            if let Some(handle) = self.pty_handles.get_mut(session_id) {
                // Drain output
                let chunks = handle.try_read();
                let has_output = !chunks.is_empty();

                for chunk in &chunks {
                    self.log_store.append(*session_id, chunk);
                    let _ = persistence::append_log(log_path, chunk);
                }

                if has_output {
                    self.last_output_at.insert(*session_id, Instant::now());

                    // Check for needs_input patterns (only for Running sessions)
                    if *status == SessionStatus::Running {
                        if let Some(line) = self.log_store.last_non_empty_line(session_id) {
                            if self.needs_input_detector.check(line) {
                                needs_input_detected.push(*session_id);
                            }
                        }
                    }
                }

                // Check exit
                if let Some((success, code)) = handle.check_exit() {
                    finished.push((*session_id, success, code));
                }
            }
        }

        // Check timeout for Running sessions without recent output
        let timeout = Duration::from_secs(self.config.needs_input_timeout_sec);
        for (session_id, _, status) in &active {
            if *status == SessionStatus::Running {
                if let Some(last) = self.last_output_at.get(session_id) {
                    if last.elapsed() > timeout {
                        needs_input_detected.push(*session_id);
                    }
                }
            }
        }

        // Transition to NeedsInput
        let mut state_changed = false;
        for id in &needs_input_detected {
            if let Some(session) = self.sessions.iter_mut().find(|s| s.id == *id) {
                if session.status == SessionStatus::Running {
                    let _ = session.transition_to(SessionStatus::NeedsInput);
                    state_changed = true;
                }
            }
        }

        // Handle finished sessions
        if !finished.is_empty() {
            for (id, success, code) in &finished {
                self.pty_handles.remove(id);
                self.last_output_at.remove(id);
                if let Some(session) = self.sessions.iter_mut().find(|s| s.id == *id) {
                    // NeedsInput → Running first if needed for valid transition
                    if session.status == SessionStatus::NeedsInput {
                        let _ = session.transition_to(SessionStatus::Running);
                    }
                    if *success {
                        let _ = session.transition_to(SessionStatus::Done);
                    } else {
                        let _ = session.transition_to(SessionStatus::Failed);
                    }
                    session.exit_code = Some(*code as i32);
                    session.pty_pid = None;
                }
            }
            state_changed = true;
        }

        if state_changed {
            self.save();
        }

        Cmd::None
    }

    fn handle_create_dialog(&mut self, key: KeyEvent) -> Cmd<Msg> {
        match key.code {
            KeyCode::Escape => {
                self.creating_session = false;
            }
            KeyCode::Enter => {
                if self.create_step == CreateStep::Name {
                    self.create_step = CreateStep::Path;
                } else {
                    let name = if self.create_name.is_empty() {
                        format!("session-{}", self.sessions.len() + 1)
                    } else {
                        self.create_name.clone()
                    };
                    let path = if self.create_path.is_empty() {
                        std::env::current_dir().unwrap_or_default()
                    } else {
                        std::path::PathBuf::from(&self.create_path)
                    };

                    if path.is_dir() {
                        let session = Session::new(name, path, &self.config.logs_root_path);
                        self.sessions.push(session);
                        self.active_session = self.sessions.len() - 1;
                        self.save();
                    }
                    self.creating_session = false;
                }
            }
            KeyCode::Char(c) => {
                if self.create_step == CreateStep::Name {
                    self.create_name.push(c);
                } else {
                    self.create_path.push(c);
                }
            }
            KeyCode::Backspace => {
                if self.create_step == CreateStep::Name {
                    self.create_name.pop();
                } else {
                    self.create_path.pop();
                }
            }
            _ => {}
        }
        Cmd::None
    }

    fn handle_rename_dialog(&mut self, key: KeyEvent) -> Cmd<Msg> {
        match key.code {
            KeyCode::Escape => {
                self.renaming = false;
            }
            KeyCode::Enter => {
                if let Some(session) = self.sessions.get_mut(self.active_session) {
                    if !self.rename_text.is_empty() {
                        session.name = self.rename_text.clone();
                        self.save();
                    }
                }
                self.renaming = false;
            }
            KeyCode::Char(c) => {
                self.rename_text.push(c);
            }
            KeyCode::Backspace => {
                self.rename_text.pop();
            }
            _ => {}
        }
        Cmd::None
    }
}

impl Model for AppState {
    type Message = Msg;

    fn init(&mut self) -> Cmd<Msg> {
        Cmd::None
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Key(key) => self.handle_key(key),
            Msg::PtyPollTick => self.handle_pty_poll(),
            Msg::Noop => Cmd::None,
        }
    }

    fn view(&self, frame: &mut Frame) {
        let area = Rect::from_size(frame.buffer.width(), frame.buffer.height());

        // Top-level: body | input_area | status_bar
        let rows = Flex::vertical()
            .constraints([
                Constraint::Min(10),
                Constraint::Fixed(3),
                Constraint::Fixed(1),
            ])
            .split(area);
        let body = rows[0];
        let input_area = rows[1];
        let status_area = rows[2];

        // Body: sidebar | center | log_panel
        let cols = Flex::horizontal()
            .constraints([
                Constraint::Fixed(28),
                Constraint::Percentage(40.0),
                Constraint::Min(30),
            ])
            .split(body);
        let sidebar = cols[0];
        let center = cols[1];
        let log_area = cols[2];

        // Sidebar: session_list | dir_tree
        let sidebar_rows = Flex::vertical()
            .constraints([Constraint::Percentage(40.0), Constraint::Percentage(60.0)])
            .split(sidebar);
        let session_list_area = sidebar_rows[0];
        let dir_tree_area = sidebar_rows[1];

        // Render each panel
        ui::session_list::render(self, frame, session_list_area, self.active_panel == Panel::SessionList);
        ui::dir_tree_panel::render(self, frame, dir_tree_area, self.active_panel == Panel::DirTree);
        ui::file_panel::render(self, frame, center, self.active_panel == Panel::FilePreview);
        ui::log_panel::render(self, frame, log_area, self.active_panel == Panel::Log);

        // Input bar - show dialog if active, otherwise normal input
        if self.creating_session {
            self.render_create_dialog(frame, input_area);
        } else if self.renaming {
            self.render_rename_dialog(frame, input_area);
        } else {
            ui::input_bar::render(self, frame, input_area, self.active_panel == Panel::Input);
        }

        ui::status_bar::render(self, frame, status_area);
    }

    fn subscriptions(&self) -> Vec<Box<dyn ftui_runtime::subscription::Subscription<Msg>>> {
        let has_active = self.sessions.iter().any(|s| {
            s.status == SessionStatus::Running || s.status == SessionStatus::NeedsInput
        });

        if has_active {
            vec![Box::new(Every::new(
                Duration::from_millis(50),
                || Msg::PtyPollTick,
            ))]
        } else {
            vec![]
        }
    }
}

impl AppState {
    fn render_create_dialog(&self, frame: &mut Frame, area: Rect) {
        use ftui_render::cell::PackedRgba;
        use ftui_style::Style;
        use ftui_widgets::block::Block;
        use ftui_widgets::paragraph::Paragraph;
        use ftui_widgets::Widget;

        let text = if self.create_step == CreateStep::Name {
            format!("Session name (Enter to skip): {}", self.create_name)
        } else {
            format!("Work directory (Enter to use cwd): {}", self.create_path)
        };

        let paragraph = Paragraph::new(text).block(
            Block::bordered()
                .title("New Session")
                .border_style(Style::new().fg(PackedRgba::rgb(205, 205, 0))),
        );
        paragraph.render(area, frame);
    }

    fn render_rename_dialog(&self, frame: &mut Frame, area: Rect) {
        use ftui_render::cell::PackedRgba;
        use ftui_style::Style;
        use ftui_widgets::block::Block;
        use ftui_widgets::paragraph::Paragraph;
        use ftui_widgets::Widget;

        let text = format!("New name: {}", self.rename_text);
        let paragraph = Paragraph::new(text).block(
            Block::bordered()
                .title("Rename Session")
                .border_style(Style::new().fg(PackedRgba::rgb(205, 205, 0))),
        );
        paragraph.render(area, frame);
    }
}
