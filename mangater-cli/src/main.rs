mod cli;

use clap::Parser;

use crate::cli::Cli;
use mangater_cli::entity::LogLevel;
use mangater_cli::cmd;

use tracing_subscriber::{EnvFilter};
use tracing::{debug, info, warn, error};

fn init_tracing(log_level: &LogLevel) {
    // default is info level
    // let filter = EnvFilter::try_from_default_env()
    //     .unwrap_or_else(|_| EnvFilter::new("info"));

    // use the provided argument instead of from env var
    let filter = EnvFilter::new(log_level.to_string());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    init_tracing(&cli.log_level);
    debug!("CLI: {:?}", cli);
    debug!("Config mode: {:?}", cli.config_mode);
    debug!("Config file: {:?}", cli.config);
    debug!("Log level: {:?}", cli.log_level);
    
    match cli.command {
        cli::Commands::Scrap(scrap_args) => cmd::scrap(scrap_args)?,
    }

    // let config = load_config(&cli)?;

    // let mut engine = build_engine()?;

    // engine.run(config)?;

    Ok(())
}
