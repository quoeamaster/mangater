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

// fn parse_key_val<K, V>(s: &str) -> Result<(K, V), String>
// where
//     K: std::str::FromStr,
//     V: std::str::FromStr,
//     K::Err: std::fmt::Display,
//     V::Err: std::fmt::Display,
// {
//     let pos = s.find('=').ok_or_else(|| {
//         format!("invalid KEY=value: no `=` found in `{}`", s)
//     })?;

//     let key = s[..pos].parse().map_err(|e| format!("key error: {}", e))?;
//     let val = s[pos + 1..].parse().map_err(|e| format!("value error: {}", e))?;

//     Ok((key, val))
// }

fn parse_key_val_str(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;

    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

#[derive(clap::Args, Clone, Debug)]
pub struct ScrapArgs {
    /// URL to scrape (mandatory)
    #[arg(short, long)]
    pub url: String,

    /// override the config file's `core.storage.root_folder` value if provided
    #[arg(short, long)]
    pub output: Option<String>,

    /// Plugin-specific parameters (key=value), repeatable
    /// Example: --param rows=5 --param scrap_content=true
    #[arg(long = "param", value_parser = parse_key_val_str)]
    // [todo] make it mut...
    pub params: Vec<(String, String)>,
}

impl ScrapArgs {
    /// Converts the vector of `(String, String)` pairs in `params`
    /// into a `HashMap<String, String>` for convenient lookup.
    ///
    /// # Returns
    ///
    /// A `HashMap` where each key-value parameter is individually accessible.
    ///
    /// # Example
    ///
    /// ```
    /// let args = mangater_cli::entity::ScrapArgs {
    ///     url: "http://example.com".to_string(),
    ///     output: None,
    ///     params: vec![("q".to_string(), "mars".to_string()), ("limit".to_string(), "5".to_string())]
    /// };
    /// let map = args.params_map();
    /// assert_eq!(map.get("q"), Some(&"mars".to_string()));
    /// assert_eq!(map.get("limit"), Some(&"5".to_string()));
    /// ```
    pub fn params_map(self) -> std::collections::HashMap<String, String> {
        // [lesson]
        // updated the fn signature to self from &self
        // aim: reduce using clone()...

        // [obsolete]
        //self.params.iter().cloned().collect()

        self.params.into_iter().collect()
    }
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
