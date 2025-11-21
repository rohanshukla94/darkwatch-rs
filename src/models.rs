use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct KeywordConfig {
    pub domains: Vec<String>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IndicatorKind {
    Domain,
    Keyword,
    Email,
    Hash,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndicatorHit {
    pub kind: IndicatorKind,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub id: Uuid,
    pub url: String,
    pub source: String,
    pub hits: Vec<IndicatorHit>,
    pub severity: Severity,
    pub snippet: String,
    pub first_seen: DateTime<Utc>,
}
