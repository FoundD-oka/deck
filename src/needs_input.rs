use regex::Regex;

pub struct NeedsInputDetector {
    patterns: Vec<Regex>,
}

impl NeedsInputDetector {
    pub fn new() -> Self {
        let patterns = vec![
            Regex::new(r"\(y/n\)").unwrap(),
            Regex::new(r"\(Y/n\)").unwrap(),
            Regex::new(r"\(yes/no\)").unwrap(),
            Regex::new(r"(?i)\bconfirm\b.*\?").unwrap(),
            Regex::new(r"(?i)\bcontinue\s*\?").unwrap(),
            Regex::new(r"(?i)\bproceed\s*\?").unwrap(),
            Regex::new(r"(?i)\ballow\b.*\?").unwrap(),
            Regex::new(r"(?i)do you want to\b").unwrap(),
        ];
        Self { patterns }
    }

    pub fn check(&self, text: &str) -> bool {
        self.patterns.iter().any(|p| p.is_match(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detector() -> NeedsInputDetector {
        NeedsInputDetector::new()
    }

    #[test]
    fn detects_yn() {
        let d = detector();
        assert!(d.check("Continue? (y/n)"));
        assert!(d.check("Overwrite file? (Y/n)"));
        assert!(d.check("Are you sure? (yes/no)"));
    }

    #[test]
    fn detects_question_patterns() {
        let d = detector();
        assert!(d.check("Do you want to proceed?"));
        assert!(d.check("do you want to install this package?"));
        assert!(d.check("Would you like to continue?"));
        assert!(d.check("Shall we proceed?"));
    }

    #[test]
    fn detects_confirm_allow() {
        let d = detector();
        assert!(d.check("Please confirm?"));
        assert!(d.check("Allow access to filesystem?"));
    }

    #[test]
    fn ignores_normal_output() {
        let d = detector();
        assert!(!d.check("Building project..."));
        assert!(!d.check("Test passed"));
        assert!(!d.check("Compiling main.rs"));
        assert!(!d.check(""));
    }

    #[test]
    fn ignores_partial_matches() {
        let d = detector();
        // "confirm" without question mark should not match
        assert!(!d.check("Sending confirmation email"));
        // "allow" without question mark should not match
        assert!(!d.check("This will allow faster builds"));
    }
}
