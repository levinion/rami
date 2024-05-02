use clap::Subcommand;

#[derive(clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    #[command(alias = "h")]
    Http {
        #[arg(short, long, default_value_t = 0)]
        threads: usize,
        #[arg(short, long, default_value = "rami/0.2.0")]
        agent: String,
        source: String,
    },

    #[command(alias = "t")]
    Torrent { source: String },
}
