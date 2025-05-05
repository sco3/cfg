use std::error::Error;

use figment::{providers::Env, Figment};

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
struct OurConfig {
    application: String,
    asdf: String,
}

impl OurConfig {
    fn load() -> Result<OurConfig, Box<dyn Error>> {
        let cfg = Figment::new()
            .merge(Env::prefixed("").split("_"))
            .extract()?;

        return Ok(cfg);
    }
}
fn main() {
    match OurConfig::load() {
        Ok(cfg) => println!("{:?} {} {}sd", cfg, cfg.asdf, cfg.application),
        Err(e) => {
            println!("Failed to load config: {}", e);
        }
    }
    println!("{:?}", OurConfig::default())
}
