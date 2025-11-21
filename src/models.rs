#[derive(Debug, Clone)]
pub struct KeywordConfig {
    pub domains: Vec<String>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub url: String,
    pub indicator: String,   // e.g. "domain:google.com" or "keyword:password dump"
    pub severity: Severity,
}
