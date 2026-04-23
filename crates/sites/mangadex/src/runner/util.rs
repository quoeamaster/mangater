// site-mangadex - the mangadex site implementation for Mangater.
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

//! runner/util.rs provides utilities for the mangadex site implementation.

use crate::runner::model::AtHomeResponse;
use mangater_sdk::SdkError;

/// helper function to extract the chapter id from a given url.
///
/// # Arguments
/// * `url`: The URL to extract the chapter id from.
///
/// # Returns
/// * `Option<String>` the chapter id if it is found.
pub fn extract_chapter_id_from_url(url: &str) -> Option<String> {
    url.split("/chapter/")
        .nth(1)?
        .split('/')
        .next()
        .map(|s| s.to_string())
}

/// helper function to fetch the image urls from a given chapter id.
/// Contains site specific logic on fetching images url (call api server).
///
/// # Arguments
/// * `chapter_id`: The chapter id to fetch the image urls from.
///
/// # Returns
/// * `Result<Vec<String>, SdkError>` the image urls if they are fetched successfully.
/// * `Err(SdkError)` if the image urls cannot be fetched.
pub async fn fetch_image_urls(chapter_id: &str) -> Result<Vec<String>, SdkError> {
    // [todo] use mangater-sdk::util::resource::download_resource to fetch the image urls INSTEAD????
    // pretend to be curl to avoid being blocked by the server
    let client = reqwest::Client::builder()
        .user_agent("curl/7.88.1")
        .http1_only() // 🔥 critical for MangaDex
        .build()
        .map_err(|e| SdkError::Network(e.to_string()))?;

    let url = format!("https://api.mangadex.org/at-home/server/{}", chapter_id);
    tracing::debug!("api url: {}", url);

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?;

    let text = resp
        .text()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?;
    tracing::debug!("{} -> api result content: {:?}", url, text);

    let parsed: AtHomeResponse = serde_json::from_str(&text)
        .map_err(|e: serde_json::Error| SdkError::Parse(e.to_string()))?;

    let urls = parsed
        .chapter
        .data
        .iter()
        .map(|filename| {
            format!(
                "{}/data/{}/{}",
                parsed.base_url, parsed.chapter.hash, filename
            )
        })
        .collect();

    tracing::debug!("{} -> final image urls: {:?}", url, urls);
    Ok(urls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::EnvFilter;

    fn init_tracing() {
        let filter = EnvFilter::new("info");

        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .try_init();
    }

    #[test]
    fn test_extract_chapter_id_from_url() {
        assert_eq!(
            extract_chapter_id_from_url("https://mangadex.org/chapter/1234567890"),
            Some("1234567890".to_string())
        );
    }
    #[tokio::test]
    #[ignore = "this test should be run for once as poc only"]
    async fn test_fetch_image_urls() {
        init_tracing();

        let urls = fetch_image_urls("8a56a7c3-4dbf-4cc8-99db-5ff28033ff6e")
            .await
            .unwrap();
        assert!(!urls.is_empty());
        assert!(urls.len() > 0);

        tracing::debug!("urls: {:?}", urls);
    }
}
