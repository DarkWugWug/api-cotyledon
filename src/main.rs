use dotenv::dotenv;
use fastrand;
use serde::{Deserialize, Serialize};
use std::env::{self, VarError};
use tide::prelude::*; // Pulls in the json! macro.
use tide::{Body, Next, Request, Result as TideResult};

mod error;
use error::CotyledonError;
mod infra;
use infra::{Garden, PartialGarden, Plant, PLANT_TYPES};
mod sow;

const ENV_SECRET_KEY: &str = "COTYLEDON_SECRET";


fn generate_seed() -> String {
    let secret = std::iter::repeat_with(fastrand::alphanumeric)
        .take(24)
        .collect();
    println!("Generating temporary value...");
    println!("{}={}", ENV_SECRET_KEY, secret);
    println!("To keep this secret run:");
    println!("\techo {}={} >> .env", ENV_SECRET_KEY, secret);
    return secret;
}

#[derive(Clone)]
struct GardenSecrets {
    secret: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    let secret = match env::var(ENV_SECRET_KEY) {
        Ok(x) => x,
        Err(VarError::NotPresent) => generate_seed(),
        Err(e) => panic!("COTYLEDON_SEED found but was malformed! {}", e),
    };

    tide::log::with_level(tide::log::LevelFilter::Debug);
    let mut app = tide::with_state(
        GardenSecrets { secret }
    );
    app.at("/sow").post(crate::sow::sow_plant);
    app.at("/plants")
        .get(|_| async { Ok(json!({ "plants": PLANT_TYPES })) });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
