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

//! cmd/cmd_scrap.rs implements the logic for the scrap command.

use crate::entity::ConfigMode;
use crate::util::cli_engine::build_engine;

use crate::entity::ScrapArgs;

/// Scraps the URL and persists the content to the output directory.
///
/// # Parameters
/// - `args`: The arguments for the scrap command.
/// - `config_mode`: The configuration mode.
/// - `config_file`: The configuration file.
///
/// # Returns
/// - Returns a [anyhow::Result] indicating success or failure.
pub async fn scrap(
    args: ScrapArgs,
    config_mode: ConfigMode,
    config_file: Option<String>,
) -> anyhow::Result<(), anyhow::Error> {
    tracing::debug!("* Scraping URL: {:?}, Output: {:?}", args.url, args.output);

    let mut engine = build_engine(config_mode, config_file);

    match engine
        .run_scrap_workflow(
            args.clone().url,
            args.clone().output,
            //args.clone().params_map().clone(),
            args.params_map(),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!(
            "error scrapping and persisting the resources: {}",
            e
        )),
    }
}

// [todo] add logic to scrap
// implement next plugin
