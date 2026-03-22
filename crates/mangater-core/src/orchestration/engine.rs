use crate::orchestration::model::RegistryMapImplementation;
use crate::util::file_location::{generate_file_path_to_persist, generate_url_for_fetching};

use mangater_sdk::entity::{AppConfigJson5, PatternMatchResult, PatternType, Registerable};
use mangater_sdk::traits::Registry;
use mangater_sdk::util::html_parsing::{clean_html_content, parse_images};
use mangater_sdk::util::resource::{
    create_parent_folders_if_needed, download_resource, download_resource_to_file,
};
use mangater_sdk::SdkError;

use futures::stream::{self};
use futures_util::StreamExt;

use std::fs;
use std::sync::Arc;

pub struct Engine {
    registry: RegistryMapImplementation,

    /// configuration (core and plugins)
    config: Option<AppConfigJson5>,
}

impl Default for Engine {
    fn default() -> Self {
        // calling the new fn() constructor to initialize the instance
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            registry: RegistryMapImplementation::new(),
            config: None,
        }
    }

    /// return a read-only reference to the underlying registry implementation
    pub fn registry(&mut self) -> &mut dyn mangater_sdk::traits::Registry {
        &mut self.registry
    }

    pub fn config_load_from_json5_file(
        &mut self,
        config_file: String,
    ) -> Result<&AppConfigJson5, SdkError> {
        let config_content = fs::read_to_string(config_file.clone())
            .map_err(|e| SdkError::InvalidConfig(format!("{} - {}", config_file.clone(), e)))?;

        let config: AppConfigJson5 =
            json5::from_str(&config_content).map_err(|e| SdkError::InvalidConfig(e.to_string()))?;

        self.config = Some(config);

        Ok(self.config.as_ref().unwrap())
    }

    pub fn config_load_from_json_file(
        &mut self,
        config_file: String,
    ) -> Result<&AppConfigJson5, SdkError> {
        let config_content = fs::read_to_string(config_file.clone())
            .map_err(|e| SdkError::InvalidConfig(format!("{} - {}", config_file.clone(), e)))?;

        let config: AppConfigJson5 = serde_json::from_str(&config_content)
            .map_err(|e| SdkError::InvalidConfig(e.to_string()))?;

        self.config = Some(config);

        Ok(self.config.as_ref().unwrap())
    }

    fn get_max_concurrency(&self) -> usize {
        match self.config.as_ref() {
            Some(config) => {
                let mut value = config.core.max_concurrency as usize;
                if value == 0 {
                    // default to 1 means no concurrency (no parallelism, just sequential execution)
                    value = 1;
                }
                value
            }
            // default to 1 means no concurrency (no parallelism, just sequential execution)
            None => 1,
        }
    }
}

impl Engine {
    pub async fn run_scrap_workflow(
        &mut self,
        url: String,
        output_folder: Option<String>,
    ) -> Result<(), SdkError> {
        let (domain, domain_key) = self.registry.resolve_domain(url.as_str());
        // actually if no Domain found, not supported and throw an error
        if domain.is_none() {
            return Err(SdkError::Unsupported(url.to_string()));
        }
        if let Some(domain) = domain {
            let patterns = domain.get_domain_registerable().matcher.match_patterns(url.as_str());
            tracing::info!("patterns: {:?}", patterns);

            // in case the output folder is provided, override the config file's `core.storage.root_folder` value
            if let Some(output_folder) = output_folder {
                self.config.as_mut().unwrap().core.storage.root_folder = output_folder.clone();
                tracing::info!("** output folder overridden: {}", output_folder);
            }

            // check the patterns and check if need to scrap OR the content already ready for storage...
            self.scrap_and_persist(
                url,
                domain_key,
                &patterns,
                &domain.get_domain_registerable(),
            )
            .await?;
        }
        Ok(())
    }

    async fn scrap_and_persist(
        &self,
        url: String,
        domain_key: String,
        patterns: &Vec<PatternMatchResult>,
        registry: &Registerable,
    ) -> Result<(), SdkError> {
        // scrapping non existed resources won't end up an error but a logging (warn level)
        // on the other hand, persisting having issues ends up a SdkError

        // get the content of the url... first
        let url_content = download_resource(url.clone(), None)
            .await
            .map_err(|e| SdkError::NotFound(e.to_string()))?;

        let url_content = String::from_utf8(url_content).map_err(|e| {
            SdkError::Parse(format!(
                "{} contents could not be parsed into string -> {}",
                url.clone(),
                e
            ))
        })?;

        let url_content = Arc::new(url_content);
        let url_param = Arc::new(url.clone());
        let domain_key_arc = Arc::new(domain_key.clone());

        let results: Vec<Result<(), SdkError>> = stream::iter(patterns)
            .map(|pattern| {
                let url_content_closure = url_content.clone();
                let url_param_closure = url_param.clone();
                let domain_key_closure = domain_key_arc.clone();
                let root_folder = self
                    .config
                    .as_ref()
                    .unwrap()
                    .core
                    .storage
                    .root_folder
                    .clone();

                async move {
                    match pattern.pattern_type {
                        PatternType::Pagination => {
                            tracing::warn!("pagination pattern is not supported yet");
                            Ok(())
                        }
                        PatternType::Content => {
                            // call helper function to persist the content...
                            scrap_and_persist_content(
                                root_folder,
                                url_content_closure.as_str(),
                                url_param_closure.as_str(),
                                pattern,
                                registry,
                            )
                            .await?;

                            Ok(())
                        }
                        PatternType::ScrapedContent => {
                            tracing::warn!("scraped content pattern is not supported yet");
                            Ok(())
                        }
                        PatternType::Others => {
                            tracing::warn!("others pattern is not supported yet");
                            Ok(())
                        }
                        PatternType::ActualUri => {
                            scrap_provided_uri_and_persist(
                                root_folder,
                                url_param_closure.as_str(),
                                pattern,
                                domain_key_closure.to_string(),
                                registry
                            ).await?;

                            Ok(())
                        }
                        // [TODO] extract out for unit test and maintenance concerncs
                        PatternType::Resource => {
                            // calling the helper function
                            scrap_and_persist_resource(
                                root_folder.clone(),
                                pattern.pattern.as_str(),
                                &url_content_closure,
                                &url_param_closure,
                                domain_key_closure.to_string(),
                                pattern,
                                registry,
                            )
                            .await?;

                            Ok(())
                        }
                    }
                }
            })
            .buffer_unordered(self.get_max_concurrency())
            .collect::<Vec<_>>()
            .await;

        if let Some(Err(e)) = results.into_iter().find(Result::is_err) {
            tracing::error!(
                "error scrapping and persisting the resources (please note that it could be partial success as well): {}",
                e.to_string()
            );
            return Err(e);
        }

        Ok(())
    }
}

async fn scrap_and_persist_content(
    root_folder: String,
    url_content: &str,
    url: &str,
    pattern: &PatternMatchResult,
    registry: &Registerable,
) -> Result<(), SdkError> {
    let clean_content = clean_html_content(url_content, Some(pattern.pattern.clone()));

    match registry.storage.as_ref() {
        Some(storage) => {
            // have storage implementation to persist the content...
            let clean_content_bytes = clean_content.as_bytes().to_vec();
            let response = storage.persist(pattern, clean_content_bytes).await;

            if let Err(e) = response {
                tracing::error!("error persisting the content: {}", e.to_string());
                return Err(e);
            }
        }
        None => {
            tracing::debug!("using default storage policy to persist the html content...");
            let file_path = generate_file_path_to_persist(root_folder.clone(), url, None);
            create_parent_folders_if_needed(file_path.clone())?;
            tracing::debug!("** file_path to persist the content: {}", file_path);

            std::fs::write(file_path, clean_content.as_bytes()).map_err(SdkError::Storage)?;
        }
    }
    Ok(())
}

/// Scrapes resources from the provided HTML content based on the specified resource type and persists them.
///
/// This function currently supports scraping image resources by parsing image URLs from the input content,
/// optionally __filtering__ and __rewriting__ them according to the provided `registry` configuration. It attempts to
/// download each resource and then persists it using either the `registry`'s storage implementation or a default
/// storage policy.
///
/// # Arguments
/// * `root_folder` - The root directory where resources should be stored if no custom storage is specified.
/// * `resource_type` - The type of resource to scrape (currently only "img" is supported).
/// * `url_content` - The HTML content from which resources will be scraped.
/// * `url` - The base URL used for resolving relative resource URLs.
/// * `domain_key` - An identifier for the current domain, used in logging and warnings.
/// * `pattern` - The matched pattern specifying how resources are identified.
/// * `registry` - A reference to a registry that may contain storage, URL filtering, and URL rewriting implementations.
///
/// # Returns
/// Returns `Ok(())` on success, or an `SdkError` if resource downloading or persistence fails.
async fn scrap_and_persist_resource(
    root_folder: String,
    resource_type: &str,
    url_content: &str,
    url: &str,
    domain_key: String,
    pattern: &PatternMatchResult,
    registry: &Registerable,
) -> Result<(), SdkError> {
    match resource_type {
        "img" => {
            let mut images = parse_images(url_content.to_string());
            if let Some(url_filter) = registry.url_filter.as_ref() {
                images.retain(|html_image| url_filter.filter_url(&html_image.src.clone()));
            }
            if let Some(url_rewriter) = registry.url_rewriter.as_ref() {
                images.iter_mut().for_each(|html_image| {
                    html_image.src = url_rewriter.rewrite_url(&html_image.src.clone());
                });
            }
            tracing::debug!(
                "images parsed from url-content: {:?}, length: {}",
                images,
                images.len()
            );

            for image in images {
                let image_src_url = generate_url_for_fetching(url, &image.src.clone());
                tracing::debug!("image_src_url to be fetched: {}", image_src_url);

                let image_bytes = download_resource(image_src_url.clone(), None)
                    .await
                    .map_err(|e| SdkError::NotFound(e.to_string()));

                if let Err(e) = image_bytes {
                    tracing::warn!("error downloading the image: {}", e.to_string());
                    continue;
                }
                let image_bytes = image_bytes.unwrap();
                tracing::debug!(
                    "image_bytes downloaded: valid ? size: {}",
                    image_bytes.len()
                );

                match registry.storage.as_ref() {
                    Some(storage) => {
                        let response = storage.persist(pattern, image_bytes).await;
                        if let Err(e) = response {
                            tracing::error!("error persisting the resource: {}", e.to_string());
                            return Err(e);
                        }
                    }
                    None => {
                        tracing::debug!(
                            "utilizing default storage policy to persist the resource..."
                        );
                        let file_path = generate_file_path_to_persist(
                            root_folder.clone(),
                            image_src_url.clone().as_str(),
                            None,
                        );
                        tracing::debug!("** file_path to persist the image: {}", file_path);

                        download_resource_to_file(image_src_url.clone(), None, file_path).await?;
                    }
                }
            }
            Ok(())
        }
        _ => {
            tracing::warn!(
                "{}:{} - resource pattern {:?} is not supported yet",
                domain_key,
                url.to_string(),
                pattern.pattern
            );
            Ok(())
        }
    }
}

async fn scrap_provided_uri_and_persist(
    root_folder: String,
    src_url: &str,
    pattern: &PatternMatchResult,
    domain_key: String,
    registry: &Registerable,
) -> Result<(), SdkError> {

    let mut image_url = pattern.resource_string.as_ref().unwrap().clone();
    // any filtering?
    if let Some(url_filter) = registry.url_filter.as_ref() {
        if !url_filter.filter_url(&image_url) {
            tracing::debug!("{} -> image url filtered out: {}", domain_key, image_url);
            return Ok(());
        }
    }
    // any rewriting?
    if let Some(url_rewriter) = registry.url_rewriter.as_ref() {
        image_url = url_rewriter.rewrite_url(&image_url);
    }
    // generate url (if necessary)
    image_url = generate_url_for_fetching(src_url, &image_url);

    // download the resource
    let mut user_agent: Option<String> = None;
    if !pattern.pattern.is_empty() {
        user_agent = Some(pattern.pattern.clone());
    }
    tracing::info!("{} -> user_agent to be used: {:?}", domain_key, user_agent);

    let image_bytes = download_resource(image_url.clone(), user_agent.clone())
        .await
        .map_err(|e| SdkError::NotFound(e.to_string()));
    
    if let Err(e) = image_bytes {
        tracing::warn!("{} -> error downloading the image: {}", domain_key, e.to_string());
        return Ok(());
    }
    let image_bytes = image_bytes.unwrap();
    tracing::debug!(
        "image_bytes downloaded: valid ? size: {}",
        image_bytes.len()
    );

    // has storage implementation to persist the resource...???
    match registry.storage.as_ref() {
        Some(storage) => {
            let response = storage.persist(pattern, image_bytes).await;
            if let Err(e) = response {
                tracing::error!("error persisting the resource: {}", e.to_string());
                return Err(e);
            }
        }
        None => {
            tracing::debug!(
                "utilizing default storage policy to persist the resource..."
            );
            let file_path = generate_file_path_to_persist(
                root_folder.clone(),
                src_url,
                pattern.additoinal_params.as_ref().and_then(|params| params.get("chapter_id").cloned()),
            );
            tracing::info!("** file_path to persist the image: {}", file_path);

            download_resource_to_file(image_url.clone(), user_agent, file_path).await?;
        }
    }
    Ok(())
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

    #[tokio::test]
    async fn test_scrap_and_persist_resource() {
        init_tracing();

        struct DummyMatcher {}
        impl mangater_sdk::traits::Matcher for DummyMatcher {
            fn match_patterns(&self, _url: &str) -> Vec<PatternMatchResult> {
                vec![PatternMatchResult {
                    pattern: "img".to_string(),
                    pattern_type: PatternType::Resource,
                    resource_string: None,
                    additoinal_params: None,
                }]
            }
        }
        // copied from WikipediaInstance implementation...
        impl mangater_sdk::traits::UrlFilter for DummyMatcher {
            fn filter_url(&self, url: &str) -> bool {
                url.contains("upload.wikimedia.org")
            }
        }
        // copied from WikipediaInstance implementation...
        impl mangater_sdk::traits::UrlRewriter for DummyMatcher {
            fn rewrite_url(&self, url: &str) -> String {
                let parts: Vec<&str> = url.split("/thumb/").collect();
                if parts.len() != 2 {
                    return url.to_string();
                }
                let base = parts[0];
                let segments: Vec<&str> = parts[1].split('/').collect();
                if segments.len() < 4 {
                    return url.to_string();
                }

                let hash1 = segments[0];
                let hash2 = segments[1];
                let filename = segments[2];

                format!("{}/{}/{}/{}", base, hash1, hash2, filename)
            }
        }

        let root_folder = "./testdata/scrap_and_persist_results".to_string();
        let resource_type = "img".to_string();
        // load the content into url_content variable...
        let local_content_src =
            "testdata/scrap_and_persist_results/relational_db.local.html.txt".to_string();
        let url = "https://en.wikipedia.org/wiki/Relational_database".to_string();
        let domain_key = "wikipedia-unit-testing".to_string();
        let pattern = PatternMatchResult {
            pattern: "img".to_string(),
            pattern_type: PatternType::Resource,
            resource_string: None,
            additoinal_params: None,
        };
        let registry = Registerable {
            configurator: None,
            matcher: Arc::new(DummyMatcher {}),
            storage: None,
            url_filter: Some(Arc::new(DummyMatcher {})),
            url_rewriter: Some(Arc::new(DummyMatcher {})),
        };

        // read content for local file...
        let url_content = fs::read_to_string(local_content_src).unwrap();
        tracing::debug!("** 1. url_content: valid ? {}", url_content.len() > 100);

        let result = scrap_and_persist_resource(
            root_folder.clone(),
            &resource_type,
            &url_content,
            &url,
            domain_key,
            &pattern,
            &registry,
        )
        .await;

        // overall is ok
        // if let Err(e) = result {
        //     tracing::error!(
        //         "** 2. failed after calling scrap_and_persist_resource: {}",
        //         e.to_string()
        //     );
        //     return;
        // }
        assert!(result.is_ok());

        // check the file(s)... (at least 1 image should be present)
        let files =
            fs::read_dir(format!("{}/upload.wikimedia.org/57", root_folder).as_str()).unwrap();
        let mut img_files_cnt = 0;

        for file in files {
            let file = file.unwrap();
            let file_ext = file
                .path()
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string();
            tracing::debug!(
                "file and extension: {} - {}",
                file.path().display(),
                file_ext
            );

            match file_ext.as_str() {
                "jpeg" | "jpg" | "png" | "gif" | "bmp" | "svg" | "tiff" | "ico" | "webp" => {
                    tracing::debug!(
                        "image file found: {} - size: {}",
                        file.path().display(),
                        file.metadata().unwrap().len()
                    );
                    img_files_cnt += 1;
                }
                _ => (),
            }
        }
        assert!(
            img_files_cnt > 0,
            "at least 1 image should be present, actual count: {}",
            img_files_cnt
        );

        // 2nd part (download the html content cleaned...)
        let pattern = PatternMatchResult {
            pattern: "#mw-content-text".to_string(),
            pattern_type: PatternType::Resource,
            resource_string: None,
            additoinal_params: None,
        };
        let result =
            scrap_and_persist_content(root_folder.clone(), &url_content, &url, &pattern, &registry)
                .await;
        assert!(result.is_ok());
        // check directory... etc
        let files =
            fs::read_dir("./testdata/scrap_and_persist_results/en.wikipedia.org/wiki/").unwrap();
        let mut content_files_cnt = 0;
        for file in files {
            let file = file.unwrap();
            if file.path().is_file()
                && file
                    .file_name()
                    .to_str()
                    .unwrap()
                    .contains("Relational_database")
            {
                content_files_cnt += 1;
            }
        }
        assert_eq!(
            content_files_cnt, 1,
            "assume only 1 content file should be present, actual count: {}",
            content_files_cnt
        );
    }
}
