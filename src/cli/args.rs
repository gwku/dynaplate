use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Autoplate")]
#[command(about = "A tool to automate project templates", version = "1.0")]
pub struct Args {
    #[clap(required = true)]
    pub project_dir: PathBuf,
    #[arg(short, long)]
    pub config: PathBuf,
    #[arg(short, long, action)]
    #[clap(default_value = "false", short = 'd')]
    pub dry_run: bool,
    #[arg(short, long, action)]
    #[clap(default_value = "false", short = 'v')]
    pub verbose: bool,
}
