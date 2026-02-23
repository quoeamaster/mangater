use crate::util::engine::build_engine;

pub fn list_domains() -> anyhow::Result<Vec<String>> {
    // create engine and run list...
    let mut engine = build_engine();
    let domains = engine.registry().list_registered_domains();

    println!("Registered domain(s), count: {}", domains.len());
    for domain in &domains {
        println!("- {}\r\n", domain);
    }
    Ok(domains)
}
