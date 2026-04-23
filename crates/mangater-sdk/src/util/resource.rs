// mangater-sdk - the sdk interface for Mangater, includes traits, models and utilities.
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

//! util/resource.rs provides utilities for downloading resources.

use crate::errors::SdkError;
use futures_util::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// default user agent for the Mangater SDK
const DEFAULT_USER_AGENT: &str = "mangater-sdk/0.1 (+https://github.com/quoeamaster/mangater)";

/// helper function to download a resource from a given URI. Return the raw bytes of the resource.
/// If you need to download a resource to a file, use [download_resource_to_file] instead.
///
/// # Arguments
/// * `uri`: The URI of the resource to download.
/// * `user_agent`: The user agent to use for the request.
///
/// # Returns
/// * `Ok(Vec<u8>)` if the resource is downloaded successfully.
/// * `Err(SdkError)` if the resource cannot be downloaded.
pub async fn download_resource(
    uri: String,
    user_agent: Option<String>,
) -> Result<Vec<u8>, SdkError> {
    let user_agent = user_agent.unwrap_or(DEFAULT_USER_AGENT.to_string());
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .map_err(|e| SdkError::Network(e.to_string()))?;
    tracing::debug!("downloading resource: {}", uri);

    let response = client
        .get(uri)
        .send()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?
        .error_for_status()
        .map_err(|e| SdkError::Network(e.to_string()))?;

    // [original]
    // let body = response
    //     .text()
    //     .await
    //     .map_err(|e| SdkError::Network(e.to_string()))?;
    let body = response
        .bytes()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?;

    Ok(body.to_vec())
}

/// helper function to create parent folders if needed
///
/// # Arguments
/// * `file_path`: The path to the file.
///
/// # Returns
/// * `Ok(())` if the parent folders are created successfully.
/// * `Err(SdkError)` if the parent folders cannot be created.
pub fn create_parent_folders_if_needed(file_path: String) -> Result<(), SdkError> {
    let local_file_path = file_path.clone();
    let file_path = std::path::Path::new(&local_file_path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    tracing::debug!("folder(s) created: {}", file_path.display());
    Ok(())
}

/// helper function to download a resource to a file
///
/// # Arguments
/// * `uri`: The URI of the resource to download.
/// * `user_agent`: The user agent to use for the request.
/// * `file_path`: The path to the file.
///
/// # Returns
/// * `Ok(())` if the resource is downloaded to the file successfully.
/// * `Err(SdkError)` if the resource cannot be downloaded to the file.
pub async fn download_resource_to_file(
    uri: String,
    user_agent: Option<String>,
    file_path: String,
) -> Result<(), SdkError> {
    let user_agent = user_agent.unwrap_or(DEFAULT_USER_AGENT.to_string());
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .map_err(|e| SdkError::Network(e.to_string()))?;

    let response = client
        .get(uri.clone())
        .send()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?
        .error_for_status()
        .map_err(|e| SdkError::Network(e.to_string()))?;

    tracing::debug!(
        "downloading resource to file: {}, source is {}",
        file_path,
        uri
    );

    // create missing folder(s) if needed...
    create_parent_folders_if_needed(file_path.clone())?;

    let mut file = File::create(file_path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| SdkError::Network(e.to_string()))?;
        file.write_all(&chunk).await.map_err(SdkError::Storage)?;
    }
    file.flush().await.map_err(SdkError::Storage)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::{self};
    use futures_util::StreamExt;
    use std::fs;

    #[tokio::test]
    async fn test_download_resource() -> Result<(), SdkError> {
        let uri = "https://en.wikipedia.org/wiki/NoSQL";

        let content = download_resource(uri.to_string(), None).await?;
        assert!(!content.is_empty());
        assert_eq!(content.len() > 100000, true); // actual content is around 183886 (check /testdata/wikipedia_nosql_local.html.txt)
        println!("content length: {}", content.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_download_resource_to_file() -> Result<(), SdkError> {
        let uri = "https://en.wikipedia.org/wiki/NoSQL";
        let file_path = "testdata/resource_test_download_resource_to_file.txt";

        download_resource_to_file(uri.to_string(), None, file_path.to_string()).await?;

        let content = fs::read_to_string(file_path)?;
        assert!(!content.is_empty());
        assert_eq!(content.len() > 100000, true); // actual content is around 183886 (check /testdata/wikipedia_nosql_local.html.txt)
        println!("after-stream download - content length: {}", content.len());

        Ok(())
    }

    /// test downloading multiple resources in parallel using a stream approach.
    #[tokio::test]
    async fn test_download_resources_in_parallel() -> Result<(), SdkError> {
        // declare a struct for holding url and file_path
        struct UrlFile {
            url: String,
            file_path: String,
        }
        let urls = vec![
            UrlFile {
                url: "https://upload.wikimedia.org/wikipedia/commons/thumb/b/b7/Last.fm_software_screenshot.png/250px-Last.fm_software_screenshot.png".to_string(),
                file_path: "testdata/parallel_download/250px-Last.fm_software_screenshot.png".to_string(),
            },
            UrlFile {
                url: "https://en.wikipedia.org/static/images/icons/enwiki-25.svg".to_string(),
                file_path: "testdata/parallel_download/enwiki-25.svg".to_string(),
            },
            UrlFile {
                url: "https://en.wikipedia.org/w/resources/assets/mediawiki_compact.svg".to_string(),
                file_path: "testdata/parallel_download/mediawiki_compact.svg".to_string(),
            },
        ];

        let results = stream::iter(urls)
            .map(|url_file| async move {
                let result = download_resource_to_file(
                    url_file.url.to_string(),
                    None,
                    url_file.file_path.to_string(),
                )
                .await;
                if let Err(err) = result {
                    tracing::error!("error downloading resource: {} {:?}", url_file.url, err);
                }
            })
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await;

        for result in results {
            println!("result: {:?}", result);
        }

        Ok(())
    }
}

// preferred approach...

// let results: Vec<DownloadResult> = stream::iter(patterns)
//     .map(|pattern| async move {
//         self.process_pattern(pattern).await
//     })
//     .buffer_unordered(self.config.max_concurrency)
//     .collect()
//     .await;

// match pattern.pattern_type {
//     PatternType::PlainText => self.fetch_text(pattern).await,
//     PatternType::BinaryResource => self.fetch_binary(pattern).await,
//     PatternType::Ready => self.handle_ready(pattern).await,
// }

// /// Example: download multiple files in parallel with a Semaphore
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     use tokio::task;

//     // Limit parallelism to 5 downloads
//     let sem = Arc::new(Semaphore::new(5));
//     let urls = vec![
//         "https://example.com/cat.jpg",
//         "https://example.com/dog.jpg",
//         "https://example.com/bird.jpg",
//         "https://example.com/fish.jpg",
//         "https://example.com/rabbit.jpg",
//         "https://example.com/hamster.jpg",
//     ];

//     let mut handles = vec![];

//     for (i, url) in urls.iter().enumerate() {
//         let sem = sem.clone();
//         let url = url.to_string();
//         let path = format!("file_{}.jpg", i);

//         // Acquire a permit before spawning
//         let permit = sem.acquire_owned().await?;

//         let handle = task::spawn(async move {
//             // The permit is held until this async block ends
//             let _permit = permit;
//             match download_to_file(&url, Path::new(&path)).await {
//                 Ok(_) => println!("Downloaded {}", url),
//                 Err(e) => eprintln!("Failed {}: {}", url, e),
//             }
//         });

//         handles.push(handle);
//     }

//     // Wait for all tasks
//     for handle in handles {
//         handle.await?;
//     }

//     Ok(())
// }
