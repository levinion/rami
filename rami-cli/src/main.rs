use clap::Parser;
use cli::Cli;
use rami_core::Downloader;

mod cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let threads = if cli.threads == 0 {
        num_cpus::get() / 2
    } else {
        cli.threads
    };
    let downloader = Downloader::new(&cli.url, &cli.agent, threads);
    downloader.download().await.unwrap();
}
