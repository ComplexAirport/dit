pub struct IgnoredPatterns {
    // Represents a list of patterns which are ignored by dit
    pub patterns: Vec<String>
}

impl IgnoredPatterns {
    pub fn from(patterns: Vec<String>) -> Self {
        Self { patterns }
    }
}
