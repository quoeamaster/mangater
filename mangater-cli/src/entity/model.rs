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

    /// Output file path
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

impl LogLevel {
    pub fn to_string(&self) -> String {
        match self {
            LogLevel::Trace => String::from("trace"),
            LogLevel::Debug => String::from("debug"),
            LogLevel::Info => String::from("info"),
            LogLevel::Warn => String::from("warn"),
            LogLevel::Error => String::from("error"),
        }
    }
}
