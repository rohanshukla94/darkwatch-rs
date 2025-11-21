mod config;
mod crawler;
mod models;
mod tor_client;

use std::time::Duration;

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

use crate::config::load_config;
use crate::crawler::generic::crawl_once;
use crate::models::{Finding, KeywordConfig, Severity};
use crate::tor_client::build_client;

fn print_finding_cli(f: &Finding) {
    println!();
    println!("================= 🔥 FINDING 🔥 =================");
    println!("ID:        {}", f.id);
    println!("URL:       {}", f.url);
    println!("Source:    {}", f.source);
    println!(
        "Severity:  {}",
        match f.severity {
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
        }
    );
    println!("Hits:");
    for h in &f.hits {
        println!("  - {:?}: {}", h.kind, h.value);
    }
    println!("Snippet:");
    println!("-----------------------------------------------");
    println!("{}", f.snippet);
    println!("-----------------------------------------------");
    println!("First seen: {}", f.first_seen);
    println!("===============================================");
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    info!("🕳️ DarkWebScan › Booting Dark web scan..");

    let cfg = load_config();
    info!(
        "config loaded: use_tor={}, tor={}",
        cfg.use_tor, cfg.tor_socks_addr
    );

    let client = build_client(cfg.use_tor, &cfg.tor_socks_addr)?;
    info!("http client created");

    // if let Some(url) = cfg.sites.first() {
    //     info!("crawling {}", url);
    //     if let Err(e) = crawl_once(&client, url).await {
    //         eprintln!("crawl failed: {e:?}");
    //     }
    // } else {
    //     info!("no sites configured");
    // }

    let interval = Duration::from_secs(cfg.interval_secs);

    info!("🕳️ DarskWebScan › Crawling TOR hidden services…");
    loop {
        for url in &cfg.sites {
    info!("crawling {url}");
    match crawl_once(&client, url, &cfg.org).await {
        Ok(findings) => {
            if findings.is_empty() {
                info!("🔍 DarkWebScan › No org indicators found on {}", url);
            } else {
                for f in findings {
                    // log a short line
                    match f.severity {
                        Severity::High => {
                            info!(
                                "🚨 DarkWebScan › HIGH alert on {} — {} hits",
                                f.url,
                                f.hits.len()
                            );
                        }
                        Severity::Medium => {
                            info!(
                                "⚠️ DarkWebScan › Medium alert on {} — {} hits",
                                f.url,
                                f.hits.len()
                            );
                        }
                        Severity::Low => {
                            info!(
                                "ℹ️ DarkWebScan › Low alert on {} — {} hits",
                                f.url,
                                f.hits.len()
                            );
                        }
                    }

                    print_finding_cli(&f);
                }
            }
        }
        Err(e) => {
            eprintln!("crawl failed for {url}: {e:?}");
        }
    }
}


        info!("sleeping for {:?} before next scan...", interval);
        tokio::time::sleep(interval).await;
        // info!("💬 DarkWebScan › Monitoring underground forums…");
        // info!("🧩 DarkWebScan › Parsing ransomware leak portals...");
        // info!("🛒 DarkWebScan › Tracking marketplace intel flow...");
        // info!("📡 DarkWebScan › Listening to Telegram/Discord chatter…");
        // info!("📝 DarkWebScan › Watching paste sites for dumps…");
        // info!("📁 DarkWebScan › Scraping breach databases...");
    }
    // Ok(())
}
