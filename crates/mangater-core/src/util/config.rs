// mangater-core - the engine for Mangater
// Copyright (C) 2026 Takara-Mono <quoeamaster@gmail.com>
//
// This file is dual-licensed under the terms of the MIT.
//
// You may choose either license at your option.
// For a copy of the MIT license, see <https://opensource.org/licenses/MIT>.
//
// MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use mangater_sdk::SdkError;
use std::fs;

pub const DEFAULT_ENV_FILE: &str = ".env";
pub const DEFAULT_JSON_FILE: &str = "config.json";

/// Loads environment variables from the specified `.env` file using the `dotenvy` crate.
///
/// # Arguments
///
/// * `env_file` - An `Option<String>` specifying the path to the environment file. If `None`, defaults to `DEFAULT_ENV_FILE`.
///
/// # Returns
///
/// * `Ok(None)` if the environment variables are loaded successfully.
/// * `Err(SdkError)` if the `.env` file cannot be found or loaded.
///
/// # Errors
///
/// Returns `SdkError::NotFound` if the environment file does not exist or cannot be read.
pub fn load_from_env(env_file: Option<String>) -> Result<Option<String>, SdkError> {
    // in case env_file was None; default to .env
    let env_file = env_file.unwrap_or(DEFAULT_ENV_FILE.to_string());

    // map the error to SdkError
    dotenvy::from_filename(&env_file).map_err(|e| {
        SdkError::NotFound(
            format!(
                "Failed to load environment variables, make sure the env file {env_file} exists - {e}"
            )
            .to_string(),
        )
    })?;
    // since everything is loaded into env var, nothing is supposed to be returned
    Ok(None)
}

/// Loads a JSON configuration file and returns its contents as a `String` wrapped in a `Some` variant on success.
///
/// # Arguments
///
/// * `json_file` - An `Option<String>` specifying the path to the JSON configuration file. If `None`, defaults to `DEFAULT_JSON_FILE`.
///
/// # Returns
///
/// * `Ok(Some(String))` containing the file's contents on success.
/// * `Err(SdkError)` if the file cannot be found or read.
///
/// # Errors
///
/// Returns `SdkError::NotFound` if the JSON configuration file does not exist or cannot be read.
pub fn load_from_json(json_file: Option<String>) -> Result<Option<String>, SdkError> {
    let json_file = json_file.unwrap_or(DEFAULT_JSON_FILE.to_string());

    // map the error to SdkError
    let config_in_json = fs::read_to_string(&json_file).map_err(|e| {
        SdkError::NotFound(
            format!(
                "Failed to load JSON configuration, make sure the file {json_file} exists - {e}"
            )
            .to_string(),
        )
    })?;
    // as loading a json file, the contents are supposed to be returned (contrast to load_from_env() which returns nothing)
    Ok(Some(config_in_json))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_env() {
        // .env file does not exist; hence will have error for sure
        let result = load_from_env(None);
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("Failed to load environment variables, make sure the env file"));
        // again but checking error type instead
        let result = load_from_env(None);
        match result {
            Ok(_) => panic!("Expected error (default .env file does not exist), got Ok"),
            // make sure the error type is SdkError::NotFound
            Err(SdkError::NotFound(msg)) => {
                assert!(msg.contains("Failed to load environment variables, make sure the env file"))
            }
            Err(e) => panic!("Expected SdkError::NotFound, got {:?}", e),
        }
        // again with a valid custom .env file
        let result = load_from_env(Some("testing/.env-unit-test".to_string()));
        assert!(result.is_ok());
        match std::env::var("unit.test.sequence") {
            Ok(value) => assert_eq!(value, "9801"),
            Err(e) => panic!("Expected Ok, got {:?}", e),
        }
        match std::env::var("unit.test.message") {
            Ok(value) => assert_eq!(value, "happy plugin dev"),
            Err(e) => panic!("Expected Ok, got {:?}", e),
        }
        match std::env::var("non-exist-var") {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                // [debug]
                // println!("error: {:?}", e.to_string());
                assert!(e.to_string().contains("environment variable not found"))
            }
        }
    }

    #[test]
    fn test_load_from_json() {
        // json file does not exist; hence will have error for sure (config.json does not exist)
        let result = load_from_json(None);
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("Failed to load JSON configuration, make sure the file"));
        // again but checking error type instead
        let result = load_from_json(None);
        match result {
            Ok(_) => panic!("Expected error (default config.json file does not exist), got Ok"),
            Err(SdkError::NotFound(msg)) => {
                assert!(msg.contains("Failed to load JSON configuration, make sure the file"))
            }
            Err(e) => panic!("Expected SdkError::NotFound, got {:?}", e),
        }
        // again with a valid custom config.json file
        let result = load_from_json(Some("testing/configu-unit-test.json".to_string()));
        assert!(result.is_ok());

        let config_content = result.unwrap().unwrap();

        assert!(config_content.contains("\"unit-test\": {"));
        assert!(config_content.contains("\"sequence\": \"9801\""));
        assert!(config_content.contains("\"message\": \"happy plugin dev\""));
        assert!(config_content.contains("\"ip\": \"192.168.1.100\""));
        // non existing content
        assert_eq!(config_content.contains("\"production-test\": {"), false);

        // [note] the fn would extract raw content as-is;
        // and further transformation into a struct is done by serde-json
        // instead which is up to the implementor to maintain maximum flexibility.
    }
}
