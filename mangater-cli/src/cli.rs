use clap::{Parser, Subcommand};
use mangater_cli::entity::{ConfigMode, LogLevel, ScrapArgs};

#[derive(Parser, Debug)]
#[command(
    name = "mangater",
    version = "1.0.0",
    author = "Takara-Mono <quoeamaster@gmail.com>",
    about = "Mangater - the content scrapping and management machine"
)]
pub struct Cli {
    //#[command(flatten)]
    //pub global_args: GlobalArgs,
    /// Config file path
    #[arg(global = true, short, long)]
    pub config: Option<String>,

    /// Config source
    #[arg(global = true,long, value_enum, default_value_t = ConfigMode::Json)]
    pub config_mode: ConfigMode,

    #[arg(global = true, short, long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Scrap based on the provided URL; if a supported `matcher` is found, the scrap will be performed"
    )]
    Scrap(ScrapArgs),

    #[command(about = "List all supported domains")]
    ListDomains,
}
