use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Queued,
    Running,
    NeedsInput,
    Done,
    Failed,
}

impl SessionStatus {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Queued => "○",
            Self::Running => "●",
            Self::NeedsInput => "◆",
            Self::Done => "✓",
            Self::Failed => "✗",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub root_path: PathBuf,
    pub status: SessionStatus,
    #[serde(skip)]
    pub pty_pid: Option<u32>,
    pub instruction: Option<String>,
    pub log_path: PathBuf,
    pub exit_code: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(name: String, root_path: PathBuf, logs_root: &std::path::Path) -> Self {
        let id = Uuid::new_v4();
        let log_path = logs_root.join(format!("{}.log", id));
        Self {
            id,
            name,
            root_path,
            status: SessionStatus::Queued,
            pty_pid: None,
            instruction: None,
            log_path,
            exit_code: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn can_transition_to(&self, target: &SessionStatus) -> Result<(), &'static str> {
        use SessionStatus::*;
        match (&self.status, target) {
            (Queued, Running) => Ok(()),
            (Running, Done) | (Running, Failed) | (Running, NeedsInput) => Ok(()),
            (NeedsInput, Running) => Ok(()),
            (Failed, Running) => Ok(()),
            (Done, Queued) => Ok(()),
            (Queued, Done) => Err("Cannot complete without running"),
            (Queued, NeedsInput) => Err("Cannot need input without running"),
            (Done, Running) => Err("Must go through Queued first"),
            (Done, Failed) => Err("Already completed successfully"),
            (Failed, Done) => Err("Must re-run through Running first"),
            _ => Err("Invalid state transition"),
        }
    }

    pub fn transition_to(&mut self, target: SessionStatus) -> Result<(), &'static str> {
        self.can_transition_to(&target)?;
        self.status = target;
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn make_session() -> Session {
        Session::new("test".to_string(), PathBuf::from("/tmp"), Path::new("/tmp/logs"))
    }

    #[test]
    fn valid_transitions() {
        let mut s = make_session();
        assert_eq!(s.status, SessionStatus::Queued);

        assert!(s.transition_to(SessionStatus::Running).is_ok());
        assert_eq!(s.status, SessionStatus::Running);

        assert!(s.transition_to(SessionStatus::Done).is_ok());
        assert_eq!(s.status, SessionStatus::Done);

        assert!(s.transition_to(SessionStatus::Queued).is_ok());
        assert_eq!(s.status, SessionStatus::Queued);
    }

    #[test]
    fn running_to_needs_input() {
        let mut s = make_session();
        s.transition_to(SessionStatus::Running).unwrap();
        assert!(s.transition_to(SessionStatus::NeedsInput).is_ok());
        assert!(s.transition_to(SessionStatus::Running).is_ok());
    }

    #[test]
    fn running_to_failed_and_retry() {
        let mut s = make_session();
        s.transition_to(SessionStatus::Running).unwrap();
        s.transition_to(SessionStatus::Failed).unwrap();
        assert!(s.transition_to(SessionStatus::Running).is_ok());
    }

    #[test]
    fn prohibited_queued_to_done() {
        let s = make_session();
        assert!(s.can_transition_to(&SessionStatus::Done).is_err());
    }

    #[test]
    fn prohibited_queued_to_needs_input() {
        let s = make_session();
        assert!(s.can_transition_to(&SessionStatus::NeedsInput).is_err());
    }

    #[test]
    fn prohibited_done_to_running() {
        let mut s = make_session();
        s.transition_to(SessionStatus::Running).unwrap();
        s.transition_to(SessionStatus::Done).unwrap();
        assert!(s.can_transition_to(&SessionStatus::Running).is_err());
    }

    #[test]
    fn prohibited_done_to_failed() {
        let mut s = make_session();
        s.transition_to(SessionStatus::Running).unwrap();
        s.transition_to(SessionStatus::Done).unwrap();
        assert!(s.can_transition_to(&SessionStatus::Failed).is_err());
    }

    #[test]
    fn prohibited_failed_to_done() {
        let mut s = make_session();
        s.transition_to(SessionStatus::Running).unwrap();
        s.transition_to(SessionStatus::Failed).unwrap();
        assert!(s.can_transition_to(&SessionStatus::Done).is_err());
    }

    #[test]
    fn icons() {
        assert_eq!(SessionStatus::Queued.icon(), "○");
        assert_eq!(SessionStatus::Running.icon(), "●");
        assert_eq!(SessionStatus::NeedsInput.icon(), "◆");
        assert_eq!(SessionStatus::Done.icon(), "✓");
        assert_eq!(SessionStatus::Failed.icon(), "✗");
    }

    #[test]
    fn serialization_roundtrip() {
        let s = make_session();
        let json = serde_json::to_string(&s).unwrap();
        let s2: Session = serde_json::from_str(&json).unwrap();
        assert_eq!(s.id, s2.id);
        assert_eq!(s.name, s2.name);
        assert_eq!(s.status, s2.status);
        assert!(s2.pty_pid.is_none()); // skipped in serde
    }
}
