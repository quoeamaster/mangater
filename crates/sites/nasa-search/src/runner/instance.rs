use mangater_sdk::entity::{PatternMatchResult, PatternType, PluginContext, Registerable};
use mangater_sdk::traits::{Domain, Matcher};
use mangater_sdk::util::async_util::block_on_async;
use mangater_sdk::util::resource::download_resource;
use mangater_sdk::SdkError;

use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Arc;
//use urlencoding::encode;
use std::collections::HashMap;

use crate::runner::model::{NasaApiResponse, NasaItem};

/// for managdex domain matching, a set of static regex is used to avoid recompilation on each match.
/// eg. https://images-api.nasa.gov/search?q=mars&media_type=image
static NASA_SEARCH_REGEX: Lazy<Regex> =
    // r"^https://images-api\.nasa\.gov/search\?q=(.*)$"
    // r"^https://images-api\.nasa\.gov/(.*)$"
    Lazy::new(|| Regex::new(r"^https://images-api\.nasa\.gov/search\?q=(.*)$").unwrap());

static NASA_SEARCH_DOMAIN_KEY: Lazy<String> = Lazy::new(|| "nasa-search".to_string());

#[derive(Clone, Debug)]
pub struct NasaSearchInstance {
    pub domain_key: String,
}

impl Default for NasaSearchInstance {
    fn default() -> Self {
        Self::new()
    }
}

impl NasaSearchInstance {
    pub fn new() -> Self {
        Self {
            domain_key: NASA_SEARCH_DOMAIN_KEY.to_string(),
        }
    }
}

impl Domain for NasaSearchInstance {
    fn match_domain(&self, domain: String) -> Result<bool, SdkError> {
        Ok(NASA_SEARCH_REGEX.is_match(&domain))
    }

    fn get_domain_key(&self) -> String {
        self.domain_key.clone()
    }

    fn get_domain_registerable(&self) -> Registerable {
        Registerable {
            configurator: None,
            matcher: Arc::new(self.clone()), // matcher: Arc::new(WikipediaInstance), (if stateless, no need to clone)
            storage: None,
            url_filter: None,
            url_rewriter: None,
        }
    }
}

// [obsolete] const NASA_SEARCH_URL_IMAGE_PREFIX: &str = "https://images-api.nasa.gov/search?media_type=image";
// const NASA_SEARCH_URL_IMAGE_PREFIX_WITH_Q: &str =
//     "https://images-api.nasa.gov/search?media_type=image&q={q}";
const DEFAULT_ROWS: &str = "10";

impl Matcher for NasaSearchInstance {
    fn match_patterns(&self, url: &str, context: Option<PluginContext>) -> Vec<PatternMatchResult> {
        // context must have "q"
        // context might hve "rows", defaul to 10 if not provided

        // [lesson] used to extract the q from the context, but now it is in the url itself
        // let q = context
        //     .as_ref()
        //     .unwrap()
        //     .params
        //     .get("q")
        //     .cloned()
        //     .unwrap_or_default();
        // if q.is_empty() {
        //     tracing::warn!("q is empty - actually it is a MANDATORY param, return with empty vec![] and not crashing the program...");
        //     return vec![];
        // }
        let rows_in_string = context
            .as_ref()
            .unwrap()
            .get("rows")
            .cloned()
            .unwrap_or(DEFAULT_ROWS.to_string());

        // call the query api (NASA_SEARCH_URL_IMAGE_PREFIX)
        let response = block_on_async(async {
            // [obsolete]
            // let encoded_q = encode(q.as_str());
            // let uri =
            //     NASA_SEARCH_URL_IMAGE_PREFIX_WITH_Q.replace("{q}", encoded_q.to_string().as_str());
            // tracing::info!("to be downloaded uri: {}, original uri: {}, actual q: {}", uri, NASA_SEARCH_URL_IMAGE_PREFIX_WITH_Q, encoded_q);

            tracing::debug!("to be downloaded url: {}", url);
            download_resource(url.to_string(), None).await
        });
        if let Err(e) = response {
            tracing::warn!(
                "{} -> error downloading the response: {}",
                NASA_SEARCH_DOMAIN_KEY.to_string(),
                e.to_string()
            );
            return vec![];
        }
        let response = response.unwrap();
        let response = String::from_utf8(response).unwrap();
        tracing::debug!("response: {}", response);

        // model for the api response
        let parsed: NasaApiResponse = serde_json::from_str(&response).unwrap();
        tracing::debug!("parsed: {:?}", parsed);

        // take(rows_in_string_integer) images from the above (only take xxlarge.jpg or xxorig.tif)
        // actually 2 approaches here:
        // - directly filter here
        // - use UrlFilter trait
        // for simplicity, we will filter here directly
        let filtered_images = filter_images(parsed.collection.items);
        // expect only ~large.jpg left
        tracing::debug!("filtered_images: {:?}", filtered_images);

        // create the PatternMatchResult(s)
        let rows_integer = rows_in_string.parse::<usize>().unwrap();
        let mut results = Vec::new();
        for image in filtered_images.into_iter().take(rows_integer) {
            results.push(PatternMatchResult {
                // not applicable here...
                pattern: "".to_string(),
                pattern_type: PatternType::ActualUri,
                resource_string: Some(image.clone()),
                additoinal_params: Some(HashMap::from([("filepath".to_string(), image.clone())])),
            });
        }
        tracing::debug!("nasa-search results (~large.jpg): {:?}", results);
        results
    }
}

fn filter_images(images: Vec<NasaItem>) -> Vec<String> {
    //images.into_iter().filter(|item| item.links.iter().any(|link| link.href.contains("~large.jpg") || link.href.contains("~orig.tif"))).collect()
    let filtered_images: Vec<String> = images
        .into_iter()
        .flat_map(|item| item.links) // Turn Vec<Item> into an iterator of Links
        .filter(|link| link.href.contains("~large.jpg")) // Keep only the large ones
        .map(|link| link.href) // Extract the string
        .collect();

    filtered_images
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::EnvFilter;

    /// Initializes the tracing subscriber for logging with environment filter settings.
    ///
    /// This function sets up a default logger for tests, configuring it to:
    /// - Use the `"info"` log level (can be customized with the `RUST_LOG` env variable).
    /// - Disable logging of targets.
    /// - Enable logging of source file names and line numbers.
    ///
    /// # Usage
    /// Call once at the start of a test or main function to ensure proper logging.
    ///
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
    #[ignore]
    fn test_nasa_api_match_simulation() {
        // test once is enough (unless have bug or change in api response)
        init_tracing();

        let nasa = NasaSearchInstance::new();
        let context = PluginContext::new(std::collections::HashMap::from([(
            "q".to_string(),
            "mars".to_string(),
        )]));
        let results = nasa.match_patterns("never_use_could_be_any_value", Some(context));
        tracing::debug!("results: {:?}", results);
        assert!(!results.is_empty());
        assert!(results.len() > 0);

        // ver if all links are valid (~large.jpg)
        for result in results {
            let link = result.resource_string.unwrap();
            assert!(link.contains("~large.jpg"));
        }
    }
}
