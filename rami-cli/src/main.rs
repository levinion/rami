use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use rami_core::Downloader;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Http {
            threads,
            agent,
            source,
        } => {
            let threads = if threads == 0 {
                num_cpus::get() / 2
            } else {
                threads
            };
            let downloader = Downloader::http_client(&source, &agent, threads);
            downloader.download().await?;
        }
        Command::Torrent { source } => {
            let downloader = Downloader::bittorrent_client(&source);
            downloader.download().await?;
        }
    }
    Ok(())
}
