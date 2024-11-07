use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Autoplate")]
#[command(about = "A tool to automate project templates", version = "1.0")]
pub struct Args {
    #[clap(required = true)]
    pub config: PathBuf,

    #[clap(short = 'd', long)]
    pub working_dir: Option<PathBuf>,

    #[clap(short = 't', long, default_value_t, value_enum)]
    pub config_type: ConfigType,

    #[clap(short = 'c', long, action = clap::ArgAction::SetTrue)]
    pub clean: Option<bool>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Default)]
pub enum ConfigType {
    #[default]
    Yaml,
    Json,
}
