// mangater-sdk - the interface for Mangater
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

use crate::errors::SdkError;
use serde_json::Value;
use std::collections::HashMap;

/// The `Config` trait defines an interface for loading configuration required by
/// an implementation of the Mangater SDK service. Implementation may choose the
/// actual configuration source and format (e.g. file, environment variable, remote endpoint).
///
/// Note that this implementation is ONLY required if you have your own configuration logics behind the scenes.
/// Take an example, instead of from env variables OR config file(s),
/// you are expecting to get config data stored in database / Redis - hence need your custom implementation.
pub trait Config: Send + Sync {
    /// Loads the configuration and returns it as a `String` wrapped in an `Option` on success,
    /// or a `SdkError` on failure.
    fn load(&mut self, raw_config_values: HashMap<String, Value>) -> Result<(), SdkError>;

    // /// Retrieves a specific configuration value by its string key.
    // /// The value is an Optional String, actual type conversion is up to the implementor.
    // ///
    // /// # Parameters
    // /// - `key`: The string slice representing the configuration setting to be retrieved.
    // ///
    // /// # Returns
    // /// - `Ok(Some(value))` containing the value if the specified key exists.
    // /// - `Ok(None)` if the key does not exist in the configuration source.
    // /// - `Err(SdkError)` if there is an error during the retrieval process.
    // ///
    // /// # Example
    // /// ```ignore
    // /// let value = config.config_by_key("api_token")?;
    // /// if let Some(token) = value {
    // ///     println!("API token is: {}", token);
    // /// }
    // /// ```
    // fn config_by_key(&self, key: &str) -> Result<Option<String>, SdkError>;
}
