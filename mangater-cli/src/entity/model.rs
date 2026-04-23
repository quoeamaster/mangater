// mangater-cli - the CLI for Mangater
// Copyright (C) 2026 Takara-Mono <quoeamaster@gmail.com>
//
// For a copy of the MIT license, see <https://opensource.org/licenses/MIT>.
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! entity/model.rs defines the data structures for the CLI.

use std::fmt;

use clap::ValueEnum;

/// Parses a string in the format "KEY=VALUE" into a tuple of `(String, String)`.
///
/// # Parameters
/// - `s`: The input string to parse.
///
/// # Returns
/// A tuple of `(String, String)` containing the key and value.
///
/// # Errors
/// - Returns an error if the input string is not in the format "KEY=VALUE".
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

/// ConfigMode is a enum that contains the configuration modes for the CLI.
///
/// # Parameters
/// - `Json5`: The configuration mode is JSON5.
/// - `Json`: The configuration mode is JSON.
#[derive(Clone, ValueEnum, Debug)]
pub enum ConfigMode {
    Json5,
    Json,
    //Env,
}

/// LogLevel is a enum that contains the log levels for the CLI.
/// Check the [fmt::Display] implementation for the log level strings.
/// default is [LogLevel::Info].
///
/// # Parameters
/// - `Trace`: The log level is trace.
/// - `Debug`: The log level is debug.
/// - `Info`: The log level is info.
/// - `Warn`: The log level is warn.
/// - `Error`: The log level is error.
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
    /// Formats the log level as a string.
    ///
    /// # Parameters
    /// - `f`: The formatter to write the log level to.
    ///
    /// # Returns
    /// - Returns a [fmt::Result] indicating success or failure.
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
