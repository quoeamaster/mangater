use url::Url;

/// Generates a file path for persisting content, based on the provided root folder, URL, and optionally a chapter.
///
/// # Arguments
///
/// * `root_folder_path` - The root folder where the file should be stored.
/// * `url` - The URL from which to extract the domain and path segments for folder/file structure.
/// * `chapter` - Optional chapter name to include in the generated file path.
///
/// # Returns
///
/// A `String` representing the path where the file should be persisted. The format is:
///
/// - `{root_folder}/{domain_key}/{last_path_segment}/[chapter/]source_file_name`
///
/// If the URL does not have enough path segments, sensible defaults will be used for missing components.
///
/// # Example
///
/// ```
/// let path = crate::mangater_core::util::file_location::generate_file_path_to_persist(
///     "./data".to_string(),
///     "https://example.com/chapter1/page1/image.jpg",
///     Some("chapter1".to_string()),
///     Some("image.jpg".to_string()),
/// );
/// assert_eq!(path, "./data/example.com/page1/chapter1/image.jpg");
/// ```
pub fn generate_file_path_to_persist(
    root_folder_path: String,
    url: &str,
    chapter: Option<String>,
    filepath: Option<String>,
) -> String {
    // {config.core.storage.root_folder}/{last-part-of-url}/[chapter]/{source_filename}

    tracing::debug!("generating file path to persist for url: {}", url);

    let url_parts = Url::parse(url).ok().unwrap();
    let domain_key = url_parts.domain().unwrap().to_string();

    let url_path_segments: Vec<_> = url_parts.path_segments().unwrap().collect();
    let last_part_of_url: String;
    let mut source_file_name: String = "".to_string();

    // does the filepath provided?
    if let Some(filepath) = filepath {
        // extract only the last segment
        let filepath_parts: Vec<_> = filepath.split('/').collect();
        if !filepath_parts.is_empty() {
            source_file_name = filepath_parts[filepath_parts.len() - 1].to_string();
        } else {
            source_file_name = "".to_string();
        }
    }
    tracing::debug!(
        "{} -> url parts length {} and actual content: {:?}, source file name: {}, is empty: {}",
        url,
        url_path_segments.len(),
        url_path_segments,
        source_file_name,
        source_file_name.is_empty()
    );

    if url_path_segments.len() > 2 {
        // At least 3 segments (ignoring the domain)
        last_part_of_url = url_path_segments[url_path_segments.len() - 2].to_string();
        if source_file_name.is_empty() {
            source_file_name = url_path_segments[url_path_segments.len() - 1].to_string();
        }
    } else {
        tracing::debug!(
            "url {} - does not have enough segments to generate the file path, it might not be an error, but definitely not the best scenario",
            url
        );
        if url_path_segments.len() == 2 {
            // Only 2 segments
            last_part_of_url = url_path_segments[0].to_string();
            if source_file_name.is_empty() {
                source_file_name = url_path_segments[1].to_string();
            }
        } else if url_path_segments.len() == 1 {
            last_part_of_url = "".to_string();
            if source_file_name.is_empty() {
                source_file_name = url_path_segments[0].to_string();
            }
            // [lesson] extreme case, both last part of url and source file name are empty, use a default value
            if source_file_name.is_empty() {
                source_file_name = "default_file".to_string();
            }
        } else {
            last_part_of_url = "".to_string();
            if source_file_name.is_empty() {
                source_file_name = "default_file".to_string();
            }
        }
    }
    tracing::debug!(
        "last part of url: {}, source file name: {}",
        last_part_of_url,
        source_file_name
    );

    // Use .replace() to avoid double slashes in the path due to empty segments
    match chapter {
        Some(chapter) => {
            // only works if the chapter value provided is not empty and Some()
            if !chapter.is_empty() {
                format!(
                    "{}/{}/{}/{}/{}",
                    root_folder_path, domain_key, last_part_of_url, chapter, source_file_name
                )
                .replace("//", "/")
            } else {
                format!(
                    "{}/{}/{}/{}",
                    root_folder_path, domain_key, last_part_of_url, source_file_name
                )
                .replace("//", "/")
            }
        }
        None => format!(
            "{}/{}/{}/{}",
            root_folder_path, domain_key, last_part_of_url, source_file_name
        )
        .replace("//", "/"),
    }
}

/// Generates a fully qualified URL for fetching a resource, such as an image or script,
/// given the base (page) URL and the resource's `src` attribute.
///
/// If the `resource_src` is already an absolute URL (starting with "http://" or "https://"),
/// it is returned as is. Otherwise, this function combines the scheme and domain from
/// `resource_url` with the `resource_src` path, ensuring a valid full URL is returned.
///
/// # Arguments
///
/// * `resource_url` - The base URL of the page where the resource is referenced.
/// * `resource_src` - The value of the resource's `src` attribute, which may be absolute or relative.
///
/// # Returns
///
/// A `String` containing the fully qualified URL for fetching the resource.
///
/// # Example
/// ```
/// use crate::mangater_core::util::file_location::generate_url_for_fetching;
///
/// let absolute = generate_url_for_fetching(
///     "https://example.com/path/page.html",
///     "https://cdn.example.com/img/cover.jpg"
/// );
/// assert_eq!(absolute, "https://cdn.example.com/img/cover.jpg");
///
/// let relative = generate_url_for_fetching(
///     "https://example.com/path/page.html",
///     "img/cover.jpg"
/// );
/// assert_eq!(relative, "https://example.com/img/cover.jpg");
/// ```
pub fn generate_url_for_fetching(resource_url: &str, resource_src: &str) -> String {
    tracing::debug!(
        "resource_url: {}, resource_src: {}",
        resource_url,
        resource_src
    );

    // If resource_src is already a full URL, return it directly.
    if resource_src.starts_with("https://") || resource_src.starts_with("http://") {
        resource_src.to_string()
    } else if resource_src.starts_with("//") {
        // assume https://
        format!("https://{}", resource_src)
    } else {
        // Otherwise, build an absolute URL using resource_url's scheme and domain.
        let url_parts = Url::parse(resource_url).ok().unwrap();
        format!(
            "{}://{}",
            url_parts.scheme(),
            format!("{}/{}", url_parts.domain().unwrap(), resource_src).replace("//", "/")
        )
    }
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
    fn test_generate_file_path_to_persist() {
        init_tracing();

        struct TestCase {
            sequence: u32,
            root_folder_path: String,
            url: String,
            chapter: Option<String>,
            expected_file_path: String,
        }
        let test_cases = vec![
            TestCase {
                sequence: 1,
                root_folder_path: "./testing".to_string(),
                url: "https://www.example.com/chapter1/page1/image.jpg".to_string(),
                chapter: Some("chapter1".to_string()),
                expected_file_path: "./testing/www.example.com/page1/chapter1/image.jpg"
                    .to_string(),
            },
            TestCase {
                sequence: 2,
                root_folder_path: "/dev/null".to_string(),
                url: "https://en.wiki.org/nosql/image.pNg".to_string(),
                chapter: None,
                expected_file_path: "/dev/null/en.wiki.org/nosql/image.pNg".to_string(),
            },
            TestCase {
                sequence: 3,
                root_folder_path: "./".to_string(),
                url: "https://en.wiki.org/nosql/index.htm".to_string(),
                chapter: None,
                expected_file_path: "./en.wiki.org/nosql/index.htm".to_string(),
            },
            TestCase {
                sequence: 4,
                root_folder_path: "./".to_string(),
                url: "https://en.wiki.org/nosql".to_string(),
                chapter: None,
                expected_file_path: "./en.wiki.org/nosql".to_string(),
            },
            TestCase {
                sequence: 5,
                root_folder_path: "./".to_string(),
                url: "https://en.wiki.org".to_string(),
                chapter: None,
                expected_file_path: "./en.wiki.org/default_file".to_string(),
            },
        ];

        for test_case in test_cases {
            let file_path = generate_file_path_to_persist(
                test_case.root_folder_path,
                &test_case.url,
                test_case.chapter,
                None,
            );
            tracing::debug!(
                "file path generated from source url {} -> {}",
                test_case.url,
                file_path
            );
            assert_eq!(
                file_path, test_case.expected_file_path,
                "test case {} failed",
                test_case.sequence
            );
        }
    }

    #[test]
    fn test_generate_url_for_fetching() {
        init_tracing();

        struct TestCase {
            name: String,
            url: String,
            resource_src: String,
            expected_url: String,
        }
        let test_cases = vec![
            TestCase {
                name: "test_01".to_string(),
                url: "https://www.example.com".to_string(),
                resource_src: "image.jpg".to_string(),
                expected_url: "https://www.example.com/image.jpg".to_string(),
            },
            TestCase {
                name: "test_02".to_string(),
                url: "whatever-value".to_string(),
                resource_src: "https://www.example.com/image.jpg".to_string(),
                expected_url: "https://www.example.com/image.jpg".to_string(),
            },
        ];

        for test_case in test_cases {
            let url = generate_url_for_fetching(&test_case.url, &test_case.resource_src);
            assert_eq!(
                url, test_case.expected_url,
                "test case {} failed",
                test_case.name
            );
        }
    }
}
