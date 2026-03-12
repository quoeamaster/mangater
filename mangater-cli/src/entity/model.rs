use std::fmt;

use clap::ValueEnum;

// #[derive(clap::Args, Clone, Debug)]
// pub struct GlobalArgs {
//     /// Config file path
//     #[arg(short, long)]
//     pub config: Option<String>,

//     /// Config source
//     #[arg(long, value_enum, default_value_t = ConfigMode::Json)]
//     pub config_mode: ConfigMode,
// }

#[derive(clap::Args, Clone, Debug)]
pub struct ScrapArgs {
    /// URL to scrape (mandatory)
    #[arg(short, long)]
    pub url: String,

    /// override the config file's `core.storage.root_folder` value if provided
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum ConfigMode {
    Json5,
    Json,
    //Env,
}

#[derive(Clone, ValueEnum, Debug, Default)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}
