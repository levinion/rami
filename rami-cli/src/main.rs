use anyhow::Result;
use clap::Parser;
use cli::Cli;
use download::{match_protocol, Protocol};
use rami_core::Downloader;

mod cli;
mod download;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let threads = if cli.threads == 0 {
        num_cpus::get() / 2
    } else {
        cli.threads
    };
    match match_protocol(&cli.source)? {
        Protocol::Http => {
            let downloader = Downloader::http_client(&cli.source, &cli.agent, threads);
            downloader.download().await?;
        }
        Protocol::Bittorrent => {
            let downloader = Downloader::bittorrent_client(&cli.source);
            downloader.download().await?;
        }
    };
    Ok(())
}
