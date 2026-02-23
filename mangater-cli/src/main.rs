mod cli;

use clap::Parser;

use crate::cli::Cli;
use mangater_cli::cmd;
use mangater_cli::entity::LogLevel;

use tracing::debug;
use tracing_subscriber::EnvFilter;

fn init_tracing(log_level: &LogLevel) {
    // default is info level
    // let filter = EnvFilter::try_from_default_env()
    //     .unwrap_or_else(|_| EnvFilter::new("info"));

    // use the provided argument instead of from env var
    let filter = EnvFilter::new(log_level.to_string());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .init();
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    init_tracing(&cli.log_level);
    debug!("CLI: {:?}", cli);
    debug!("Config mode: {:?}", cli.config_mode);
    debug!("Config file: {:?}", cli.config);
    debug!("Log level: {:?}", cli.log_level);

    // match the sub-command and execute the corresponding code logics
    match cli.command {
        cli::Commands::Scrap(scrap_args) => cmd::scrap(scrap_args)?,
        cli::Commands::ListDomains => {
            match cmd::list_domains(cli.config_mode, cli.config) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            return Ok(());
        }
    }

    // let config = load_config(&cli)?;

    // let mut engine = build_engine()?;

    // engine.run(config)?;

    Ok(())
}
