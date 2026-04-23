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

//! cmd/cmd_list_domains.rs implements the logic for the list-domains command.
//! Lists all supported domains from the engine's registry.

use crate::entity::ConfigMode;
use crate::util::cli_engine::build_engine;

pub fn list_domains(
    config_mode: ConfigMode,
    config_file: Option<String>,
) -> anyhow::Result<Vec<String>> {
    // create engine and run list...
    let mut engine = build_engine(config_mode, config_file);
    let domains = engine.registry().list_registered_domains();

    println!("Registered domain(s), count: {}", domains.len());
    for domain in &domains {
        println!("- {}", domain);
    }
    Ok(domains)
}
