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

mod cli;

use clap::Parser;

use crate::cli::Cli;
use mangater_cli::cmd;
use mangater_cli::entity::LogLevel;

use tracing::debug;
use tracing_subscriber::EnvFilter;

fn init_tracing(log_level: &LogLevel) {
    // default is info level
    // let filter = EnvFilter::try_from_default_env()
    //     .unwrap_or_else(|_| EnvFilter::new("info"));

    // use the provided argument instead of from env var
    let filter = EnvFilter::new(log_level.to_string());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .try_init()
        .unwrap_or_else(|e| panic!("Failed to initialize tracing: {}", e));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    init_tracing(&cli.log_level);
    debug!("CLI: {:?}", cli);
    debug!("Config mode: {:?}", cli.config_mode);
    debug!("Config file: {:?}", cli.config);
    debug!("Log level: {:?}", cli.log_level);

    // match the sub-command and execute the corresponding code logics
    match cli.command {
        cli::Commands::Scrap(scrap_args) => {
            match cmd::scrap(scrap_args, cli.config_mode, cli.config).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            return Ok(());
        }
        cli::Commands::ListDomains => {
            match cmd::list_domains(cli.config_mode, cli.config) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            return Ok(());
        }
    }
}
