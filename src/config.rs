use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};

use crate::models::KeywordConfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub use_tor: bool,
    pub tor_socks_addr: String,
    pub interval_secs: u64,
    pub org: KeywordConfig,
    pub sites: Vec<String>,
}

pub fn load_config() -> AppConfig {
    // 1. Read file
    let path = Path::new("config.toml");
    let contents = fs::read_to_string(path)
        .expect("failed to read config.toml");

    println!("RAW config.toml:\n{}\n-----", contents);

    // 2. Parse into generic TOML value
    let value: toml::Value = toml::from_str(&contents)
        .expect("failed to parse config.toml as TOML at all");

    let root = value
        .as_table()
        .expect("config.toml root is not a table somehow");

    // Debug: show top-level keys so we SEE what TOML sees
    println!(
        "Top-level keys in config.toml: {:?}",
        root.keys().collect::<Vec<_>>()
    );

    // 3. Extract primitives with defaults
    let use_tor = root
        .get("use_tor")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let tor_socks_addr = root
        .get("tor_socks_addr")
        .and_then(|v| v.as_str())
        .unwrap_or("127.0.0.1:9050")
        .to_string();

    let interval_secs = root
        .get("interval_secs")
        .and_then(|v| v.as_integer())
        .unwrap_or(60) as u64;

    // 4. Extract [org] table
    let org_tbl = root
        .get("org")
        .and_then(|v| v.as_table())
        .expect("[org] table missing or not a table");

    let domains = org_tbl
        .get("domains")
        .and_then(|v| v.as_array())
        .expect("org.domains must be an array")
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();

    let keywords = org_tbl
        .get("keywords")
        .and_then(|v| v.as_array())
        .expect("org.keywords must be an array")
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();

    // 5. Extract `sites` array
    let sites_val = root
        .get("sites")
        .unwrap_or_else(|| panic!("top-level 'sites' key not found in config.toml"));

    let sites_arr = sites_val
        .as_array()
        .unwrap_or_else(|| panic!("'sites' must be an array, e.g. sites = [\"https://...\"]"));

    let sites = sites_arr
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();

    if sites.is_empty() {
        panic!("'sites' array is empty in config.toml");
    }

    AppConfig {
        use_tor,
        tor_socks_addr,
        interval_secs,
        org: KeywordConfig { domains, keywords },
        sites,
    }
}
