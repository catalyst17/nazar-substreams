use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("EntryPoint", "abi/entrypoint.json")?
        .generate()?
        .write_to_file("src/abi/entrypoint.rs")?;

    Abigen::new("SafeV1.0.0", "abi/safe_v1.0.0.json")?
        .generate()?
        .write_to_file("src/abi/safe_v1_0_0.rs")?;

    Abigen::new("SafeV1.1.1", "abi/safe_v1.1.1.json")?
        .generate()?
        .write_to_file("src/abi/safe_v1_1_1.rs")?;

    Abigen::new("SafeV1.2.0", "abi/safe_v1.2.0.json")?
        .generate()?
        .write_to_file("src/abi/safe_v1_2_0.rs")?;

    Abigen::new("SafeV1.3.0", "abi/safe_v1.3.0.json")?
        .generate()?
        .write_to_file("src/abi/safe_v1_3_0.rs")?;

    Abigen::new("SafeV1.4.1", "abi/safe_v1.4.1.json")?
        .generate()?
        .write_to_file("src/abi/safe_v1_4_1.rs")?;

    Ok(())
}
