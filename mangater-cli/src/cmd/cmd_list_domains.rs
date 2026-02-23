use crate::entity::ConfigMode;
use crate::util::engine::build_engine;

pub fn list_domains(
    config_mode: ConfigMode,
    config_file: Option<String>,
) -> anyhow::Result<Vec<String>> {
    // create engine and run list...
    let mut engine = build_engine(config_mode, config_file);
    let domains = engine.registry().list_registered_domains();

    println!("Registered domain(s), count: {}", domains.len());
    for domain in &domains {
        println!("- {}\r\n", domain);
    }
    Ok(domains)
}
