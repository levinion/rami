#[derive(clap::Parser)]
pub struct Cli {
    pub source: String,
    #[arg(short, long, default_value_t = 0)]
    pub threads: usize,
    #[arg(short, long, default_value = "rami/0.1.0")]
    pub agent: String,
}
