use crate::entity::ConfigMode;
use crate::util::cli_engine::build_engine;

use crate::entity::ScrapArgs;

pub async fn scrap(
    args: ScrapArgs,
    config_mode: ConfigMode,
    config_file: Option<String>,
) -> anyhow::Result<(), anyhow::Error> {
    tracing::debug!("* Scraping URL: {:?}, Output: {:?}", args.url, args.output);

    let mut engine = build_engine(config_mode, config_file);

    match engine.run_scrap_workflow(args.url, args.output).await {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!(
            "error scrapping and persisting the resources: {}",
            e
        )),
    }
}

// [todo] add logic to scrap
// implement next plugin
