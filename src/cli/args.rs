use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Autoplate")]
#[command(about = "A tool to automate project templates", version = "1.0")]
pub struct Args {
    #[clap(required = true)]
    pub config: PathBuf,

    #[clap(required = true)]
    pub project_dir: PathBuf,

    #[clap(short = 't', long, default_value_t, value_enum)]
    pub config_type: ConfigType,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Default)]
pub enum ConfigType {
    #[default]
    Yaml,
    Json,
}
