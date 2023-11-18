use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("EntryPoint", "abi/entrypoint.json")?
        .generate()?
        .write_to_file("src/abi/entrypoint.rs")?;

    Ok(())
}
