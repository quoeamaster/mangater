use crate::orchestration::model::RegistryMapImplementation;
use crate::util::file_location::{generate_file_path_to_persist, generate_url_for_fetching};

use mangater_sdk::entity::{AppConfigJson5, PatternMatchResult, PatternType, Registerable};
use mangater_sdk::traits::Registry;
use mangater_sdk::util::html_parsing::parse_images;
use mangater_sdk::util::resource::download_resource;
use mangater_sdk::util::resource::download_resource_to_file;
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
        let config_content = fs::read_to_string(config_file.clone()).map_err(|e| {
            SdkError::InvalidConfig(format!("{} - {}", config_file.clone(), e.to_string()))
        })?;

        let config: AppConfigJson5 =
            json5::from_str(&config_content).map_err(|e| SdkError::InvalidConfig(e.to_string()))?;

        self.config = Some(config);

        Ok(self.config.as_ref().unwrap())
    }

    pub fn config_load_from_json_file(
        &mut self,
        config_file: String,
    ) -> Result<&AppConfigJson5, SdkError> {
        let config_content = fs::read_to_string(config_file.clone()).map_err(|e| {
            SdkError::InvalidConfig(format!("{} - {}", config_file.clone(), e.to_string()))
        })?;

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
    pub async fn run_scrap_workflow(&self, url: String) -> Result<(), SdkError> {
        let (domain, domain_key) = self.registry.resolve_domain(url.as_str());
        // actually if no Domain found, not supported and throw an error
        if domain.is_none() {
            return Err(SdkError::Unsupported(url.to_string()));
        }
        if let Some(domain) = domain {
            let patterns = domain.get_domain_registerable().matcher.match_patterns();
            tracing::info!("patterns: {:?}", patterns);

            // next...
            // check the patterns and check if need to scrap OR the content already ready for storage...
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
                e.to_string()
            ))
        })?;

        let url_content = Arc::new(url_content);
        let url_param = Arc::new(url.clone());
        let domain_key_arc = Arc::new(domain_key.clone());

        let results = stream::iter(patterns)
            .map(|pattern| {
                let url_content_closure = url_content.clone();
                let url_param_closure = url_param.clone();
                let domain_key_closure = domain_key_arc.clone();

                async move {
                    match pattern.pattern_type {
                        PatternType::Pagination => {
                            tracing::warn!("pagination pattern is not supported yet");
                            return Ok(());
                        }
                        PatternType::Content => {
                            tracing::warn!("content pattern is not supported yet");
                            return Ok(());
                        }
                        PatternType::ScrapedContent => {
                            tracing::warn!("scraped content pattern is not supported yet");
                            return Ok(());
                        }
                        PatternType::Others => {
                            tracing::warn!("others pattern is not supported yet");
                            return Ok(());
                        }
                        PatternType::Resource => {
                            match pattern.pattern.as_str() {
                                // at this moment only support image resources ('img' tag)
                                "img" => {
                                    let images = parse_images(url_content_closure.to_string());

                                    for image in images {
                                        let image_bytes = download_resource(image.src.clone(), None)
                                            .await
                                            .map_err(|e| SdkError::NotFound(e.to_string()))?;

                                        match registry.storage.as_ref() {
                                            Some(storage) => {
                                                let response = storage.persist(pattern, image_bytes).await;
                                                if let Err(e) = response {
                                                    tracing::error!("error persisting the resource: {}", e.to_string());
                                                    return Err(e);
                                                }
                                                return Ok(());
                                            }
                                            None => {
                                                tracing::info!("utilizing default storage policy to persist the resource: xxx.jpeg");
                                                let root_folder = self.config.as_ref().unwrap().core.storage.root_folder.clone();
                                                let file_path = generate_file_path_to_persist(root_folder, &image.src, None);

                                                // [todo] check if the image.src is correct...
                                                let image_src_url = generate_url_for_fetching(&url_param_closure.to_string(), &image.src.clone());
                                                download_resource_to_file(image_src_url, None, file_path).await?;

                                                return Ok(());
                                            }
                                        }
                                    }
                                    return Ok(());
                                }
                                _ => {
                                    tracing::warn!("{}:{} - resource pattern {:?} is not supported yet", domain_key_closure, url_param_closure.to_string(), pattern.pattern);
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }).buffer_unordered(self.get_max_concurrency()).collect::<Vec<_>>().await;
            
            if let Some(Err(e)) = results.into_iter().find(Result::is_err) {
                tracing::error!("error scrapping and persisting the resources: {}", e.to_string());
                return Err(e);
            }

        Ok(())
    }
}
