use anyhow::{anyhow, Result};

pub enum Protocol {
    Http,
    Bittorrent,
}

pub fn match_protocol(source: &str) -> Result<Protocol> {
    if source.starts_with("http") {
        Ok(Protocol::Http)
    } else if source.starts_with("magnet") {
        Err(anyhow!("magnet is not supported yet"))
    } else {
        Ok(Protocol::Bittorrent)
    }
}
