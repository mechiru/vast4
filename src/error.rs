/// Represents an error for parsing VAST.
#[derive(Debug)]
pub struct VastParseError {
    input: String,
}

impl VastParseError {
    pub(crate) fn new(s: String) -> Self {
        Self { input: s }
    }
}

impl std::fmt::Display for VastParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VAST parse error: {}", self.input)
    }
}

impl std::error::Error for VastParseError {}
