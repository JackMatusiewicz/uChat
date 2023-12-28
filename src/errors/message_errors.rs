pub enum MessageErrors {
    InvalidMessageData
}

impl std::fmt::Display for MessageErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InvalidMessageData")?;
        std::fmt::Result::Ok(())
    }
}

impl std::fmt::Debug for MessageErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMessageData => write!(f, "InvalidMessageData"),
        }
    }
}

impl std::error::Error for MessageErrors {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}