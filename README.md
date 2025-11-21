# darkwatch-rs

darkwatch-rs is a Rust-based dark web OSINT and threat-intelligence collector.  
It passively crawls publicly accessible Tor hidden services (via SOCKS5) and surface-web sites, extracts organization-related indicators, assigns severity levels, and exposes structured findings for defensive security and research workflows.

This project is for:
- Threat Intelligence (CTI) teams
- SOC blue teams
- Digital Risk Protection (DRP)
- Monitoring
- Lawful OSINT research

No offensive actions or exploitation are performed. The scanner only fetches publicly accessible pages.

---

## Features

- Tor-enabled HTTP crawling using a SOCKS5 proxy
- Passive OSINT collection (no scanning or exploitation)
- Keyword and domain-based indicator detection
- Severity scoring logic
- Snippet extraction for contextual understanding
- JSON-serializable findings
- Optional Axum HTTP API for programmatic access
- Async Rust (tokio + reqwest), safe, modular structure

---

## Legal and Ethical Scope

darkwatch is strictly a defensive and passive threat-intelligence tool.  
It:
- Does not exploit servers  
- Does not perform intrusive scanning  
- Does not brute-force anything  
- Only collects what is already publicly visible on the Tor network or surface web  

The tool is functionally similar to modules in commercial DRP/XDR platforms.

---

## Getting Started

### 1. Clone the repository

```
git clone git@github.com:rohanshukla94/darkwatch-rs.git
cd darkwatch-rs
```

### 2. Build

```
cargo build --release
```

### 3. Run

```
cargo run --release
```

Make sure the Tor service is running locally (default SOCKS port: 9050).

---

## Configuration

Configuration is done through `config.toml` in the project root.

Example:

```
use_tor = true
tor_socks_addr = "127.0.0.1:9050"
interval_secs = 120

sites = [
    "http://protonirockerxow.onion",
    "http://33xu4yhum2eiisxm6fntaslayop76fvaqgt3ak5dakdm3t7cub25cead.onion"
]

[org]
domains = ["example.com"]
keywords = ["confidential", "leak", "internal"]
```

---

## Output

Each matched page produces a structured finding:

- UUID
- URL scanned
- Indicator hits (domain, keyword, etc.)
- Severity (low, medium, high)
- Snippet (first 400 chars)
- Timestamp

Findings can be:
- Printed in a human-readable CLI format
- Returned as JSON via the Axum API

---

## Axum HTTP API (Optional)

Enable the HTTP API to access collected intelligence:

- GET /health  
- GET /findings  

Responses are JSON-encoded using Serde.

---

## Project Status

darkwatch currently includes:
- Configuration loader
- Tor-enabled crawler
- Keyword/domain matching
- Structured findings
- CLI reporting

TODO features:
- Axum server??
- Plugin-based crawlers for specific sites
- Persistent storage (SQLite or JSONL)
- More extractors (emails, hashes)
- Configurable severity rules
- Dashboard integration

---

## License

MIT License (or Apache 2.0, depending on your preference)
