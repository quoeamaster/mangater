use tracing::info;

use crate::entity::ScrapArgs;


pub fn scrap(args: ScrapArgs) -> anyhow::Result<()> {
    info!("* Scraping URL: {:?}, Output: {:?}", args.url, args.output);
    Ok(())
}