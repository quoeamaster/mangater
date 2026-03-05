use crate::errors::SdkError;
use futures_util::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const DEFAULT_USER_AGENT: &str = "mangater-sdk/0.1 (+https://github.com/quoeamaster/mangater)";

pub async fn download_resource(
    uri: String,
    user_agent: Option<String>,
) -> Result<Vec<u8>, SdkError> {
    let user_agent = user_agent.unwrap_or(DEFAULT_USER_AGENT.to_string());
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .map_err(|e| SdkError::Network(e.to_string()))?;

    let response = client
        .get(uri)
        .send()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?
        .error_for_status()
        .map_err(|e| SdkError::Network(e.to_string()))?;

    let body = response
        .text()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?;

    Ok(body.as_bytes().to_vec())
}

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
        .get(uri)
        .send()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?
        .error_for_status()
        .map_err(|e| SdkError::Network(e.to_string()))?;

    let mut file = File::create(file_path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| SdkError::Network(e.to_string()))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| SdkError::Storage(e))?;
    }
    file.flush()
        .await
        .map_err(|e| SdkError::Storage(e.into()))?;

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
