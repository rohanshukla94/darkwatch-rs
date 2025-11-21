use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use tracing::info;
use uuid::Uuid;

use crate::models::{Finding, IndicatorHit, IndicatorKind, KeywordConfig, Severity};

pub async fn crawl_once(
    client: &Client,
    url: &str,
    org: &KeywordConfig,
) -> Result<Vec<Finding>> {
    let resp = client.get(url).send().await?;
    let status = resp.status();
    let body = resp.text().await?;

    info!(
        target: "dark_web_scan_rs::crawler::generic",
        "Crawled {url} -> {status}, {} bytes",
        body.len()
    );

    let lower = body.to_lowercase();
    let mut hits: Vec<IndicatorHit> = Vec::new();

    for d in &org.domains {
        if lower.contains(&d.to_lowercase()) {
            hits.push(IndicatorHit {
                kind: IndicatorKind::Domain,
                value: d.clone(),
            });
        }
    }

    for kw in &org.keywords {
        if lower.contains(&kw.to_lowercase()) {
            hits.push(IndicatorHit {
                kind: IndicatorKind::Keyword,
                value: kw.clone(),
            });
        }
    }

    if hits.is_empty() {
        return Ok(Vec::new());
    }

    let severity = if hits.iter().any(|h| h.value.contains("dump") || h.value.contains("leak")) {
        Severity::High
    } else {
        Severity::Medium
    };

    let snippet: String = body.chars().take(400).collect();

    let finding = Finding {
        id: Uuid::new_v4(),
        url: url.to_string(),
        source: "generic_http".to_string(),
        hits,
        severity,
        snippet,
        first_seen: Utc::now(),
    };

    Ok(vec![finding])
}