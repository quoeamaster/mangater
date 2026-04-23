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

//! cli_engine.rs setup and config the engine for the CLI.

use mangater_core::orchestration::Engine;

use std::sync::Arc;

use crate::entity::ConfigMode;

/// Builds and configures a `mangater_core::orchestration::Engine` instance for the CLI,
/// loading config from the specified mode and file, and registering plugins according to enabled features (Cargo.toml).
///
/// # Parameters
/// - `config_mode`: The configuration format (`ConfigMode::Json5` or `ConfigMode::Json`)
/// - `config_file`: An optional path to the configuration file
///
/// # Returns
/// A fully-initialized `Engine`, loaded with configuration and plugins (wikipedia, mangadex, nasa-search) as available.
///
/// # Panics
/// Panics if configuration cannot be loaded or a plugin fails to register.
///
/// # Features
/// - `wikipedia`: Registers the Wikipedia plugin.
/// - `mangadex`: Registers the Mangadex plugin.
/// - `nasa`: Registers the NASA Search plugin.
pub fn build_engine(
    config_mode: ConfigMode,
    config_file: Option<String>,
) -> mangater_core::orchestration::Engine {
    let mut engine = Engine::new();

    // Load config from json or json5 file
    let config_file_path = config_file.unwrap_or("config.json5".to_string());
    let app_config = match config_mode {
        ConfigMode::Json5 => engine
            .config_load_from_json5_file(config_file_path)
            .unwrap(),
        ConfigMode::Json => engine.config_load_from_json_file(config_file_path).unwrap(),
    };
    tracing::debug!("overall app config: {:?}", app_config);

    // Wikipedia plugin registration (if enabled)
    #[cfg(feature = "wikipedia")]
    {
        use mangater_sdk::traits::{Config, Domain};
        use site_wikipedia::WikipediaInstance;

        let mut wikipedia = WikipediaInstance::new();
        // Plugins requiring a custom config section must implement `Config`.
        wikipedia.load(app_config.plugins.clone()).unwrap();

        // Register the Wikipedia plugin to the engine's registry.
        engine.registry().add_to_registry(
            Some(wikipedia.get_domain_key()),
            Arc::new(wikipedia.clone()),
        );
    }

    // Mangadex plugin registration (if enabled)
    #[cfg(feature = "mangadex")]
    {
        use mangater_sdk::traits::Domain;
        use site_mangadex::MangadexInstance;

        let mangadex = MangadexInstance::new();
        engine
            .registry()
            .add_to_registry(Some(mangadex.get_domain_key()), Arc::new(mangadex.clone()));
    }

    // NASA Search plugin registration (if enabled)
    #[cfg(feature = "nasa")]
    {
        use mangater_sdk::traits::Domain;
        use site_nasa_search::NasaSearchInstance;

        let nasa_search = NasaSearchInstance::new();
        engine.registry().add_to_registry(
            Some(nasa_search.get_domain_key()),
            Arc::new(nasa_search.clone()),
        );
    }

    engine
}

// [obsolete]
// pub fn setup_logging(log_level: crate::entity::LogLevel) {
//     use tracing_subscriber::EnvFilter;
//
//     let filter = match log_level {
//         crate::entity::LogLevel::Error => "error",
//         crate::entity::LogLevel::Warn => "warn",
//         crate::entity::LogLevel::Info => "info",
//         crate::entity::LogLevel::Debug => "debug",
//         crate::entity::LogLevel::Trace => "trace",
//     };
//     tracing_subscriber::fmt()
//         .with_env_filter(EnvFilter::new(filter))
//         .init();
// }
