use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc;
use std::thread::JoinHandle;

pub struct PtyHandle {
    writer: Box<dyn Write + Send>,
    output_rx: mpsc::Receiver<Vec<u8>>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
    _master: Box<dyn MasterPty>,
    _reader_thread: JoinHandle<()>,
}

impl PtyHandle {
    /// Spawn `claude` in a PTY rooted at `root_path`.
    pub fn spawn(root_path: &Path, cols: u16, rows: u16) -> anyhow::Result<Self> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let mut cmd = CommandBuilder::new("claude");
        cmd.cwd(root_path);

        let child = pair.slave.spawn_command(cmd)?;
        // Close slave end so reads on master get EOF when child exits
        drop(pair.slave);

        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let (tx, rx) = mpsc::channel();

        let _reader_thread = std::thread::spawn(move || {
            let mut reader = reader;
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if tx.send(buf[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            writer,
            output_rx: rx,
            child,
            _master: pair.master,
            _reader_thread,
        })
    }

    /// Drain all available output chunks without blocking.
    pub fn try_read(&self) -> Vec<Vec<u8>> {
        let mut chunks = Vec::new();
        while let Ok(chunk) = self.output_rx.try_recv() {
            chunks.push(chunk);
        }
        chunks
    }

    /// Send text to the PTY (e.g. user instruction + newline).
    pub fn send_input(&mut self, text: &str) -> std::io::Result<()> {
        self.writer.write_all(text.as_bytes())?;
        self.writer.flush()
    }

    /// Send Ctrl+C (0x03) to the PTY.
    pub fn send_sigint(&mut self) -> std::io::Result<()> {
        self.writer.write_all(&[0x03])?;
        self.writer.flush()
    }

    /// Non-blocking check: has the child process exited?
    /// Returns (success, exit_code) if exited.
    pub fn check_exit(&mut self) -> Option<(bool, u32)> {
        match self.child.try_wait() {
            Ok(Some(status)) => Some((status.success(), status.exit_code())),
            _ => None,
        }
    }

    /// Get the child process ID.
    pub fn process_id(&self) -> Option<u32> {
        self.child.process_id()
    }
}
