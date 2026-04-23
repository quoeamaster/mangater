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

//! cli.rs is more like a wrapper of the clap crate / library.
//! Hence it structures the CLI arguments and sub-commands.

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
    #[arg(global = true, short, long, default_value = "config.json5")]
    pub config: Option<String>,

    /// Config source
    #[arg(global = true,long, value_enum, default_value_t = ConfigMode::Json5)]
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
