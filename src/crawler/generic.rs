use anyhow::Result;
use reqwest::Client;
use tracing::info;

use crate::models::{Finding, KeywordConfig, Severity};

pub async fn crawl_once(client: &Client, url: &str, org: &KeywordConfig) -> Result<Vec<Finding>> {
    let resp = client.get(url).send().await?;
    let status = resp.status();
    let body = resp.text().await?;

    info!(
        target: "dark_web_scan_rs::crawler::generic",
        "Crawled {url} -> {status}, {} bytes",
        body.len()
    );

     let lower = body.to_lowercase();
    let mut findings = Vec::new();

    // simple severity rules for now
    let mut push_indicator = |indicator: String| {
        let sev = if indicator.contains("password") || indicator.contains("dump") {
            Severity::High
        } else {
            Severity::Medium
        };

        findings.push(Finding {
            url: url.to_string(),
            indicator,
            severity: sev,
        });
    };

    for d in &org.domains {
        if lower.contains(&d.to_lowercase()) {
            push_indicator(format!("domain:{}", d));
        }
    }

    for kw in &org.keywords {
        if lower.contains(&kw.to_lowercase()) {
            push_indicator(format!("keyword:{}", kw));
        }
    }


    Ok(findings)
}