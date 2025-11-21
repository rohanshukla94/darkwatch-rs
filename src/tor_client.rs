use anyhow::Result;
use reqwest::{Client, Proxy};

pub fn build_client(use_tor: bool, socks_addr: &str) -> Result<Client> {
    let builder = if use_tor {
        let proxy = Proxy::all(format!("socks5h://{}", socks_addr))?;
        reqwest::Client::builder().proxy(proxy)
    } else {
        reqwest::Client::builder()
    };
    Ok(builder.user_agent("dark-info-rs/0.1").build()?)
}