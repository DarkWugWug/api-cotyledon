#[macro_use]
extern crate lazy_static;

use std::env::{self, VarError};
use tide::prelude::*; // Pulls in the json! macro.
use tide::{Request, Response, StatusCode};

mod error;
use error::CotyledonError;
mod infra;
use infra::SimpleDuration;
mod plants;
use plants::{Plant, COTYLEDON_PLANTS};
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
pub struct GardenSecrets {
    secret: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();
    let secret = match env::var(ENV_SECRET_KEY) {
        Ok(x) => x,
        Err(VarError::NotPresent) => generate_seed(),
        Err(e) => panic!("COTYLEDON_SEED found but was malformed! {}", e),
    };

    tide::log::with_level(tide::log::LevelFilter::Debug);
    let mut app = tide::with_state(GardenSecrets { secret });
    app.at("/sow").post(crate::sow::sow_plant);
    app.at("/plants").get(|_| async {
        Ok(json!(COTYLEDON_PLANTS
            .keys()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()))
    });
    app.at("/plants/:name")
        .get(|req: Request<GardenSecrets>| async move {
            let plant_name = req.param("name")?;
            if COTYLEDON_PLANTS.contains_key(plant_name) {
                Ok(Response::builder(StatusCode::Ok)
                    .body(json!(COTYLEDON_PLANTS.get(plant_name).unwrap()))
                    .build())
            } else {
                Ok(CotyledonError::InvalidPlantType(plant_name.to_owned()).into())
            }
        });
    app.at("/plants/isRipe")
        .post(|mut req: Request<GardenSecrets>| async move {
            let plant: Plant = req.body_json().await?;
            if COTYLEDON_PLANTS.contains_key(plant.get_type()) {
                let elapsed: SimpleDuration = match plant.elapsed() {
                    Ok(dur) => dur.into(),
                    Err(err) => return Ok(err.into()),
                };
                let is_mature = COTYLEDON_PLANTS
                    .get(plant.get_type())
                    .unwrap()
                    .is_mature(elapsed);
                Ok(Response::builder(StatusCode::Ok)
                    .body(json!({ "elapsed": elapsed, "is_mature": is_mature }))
                    .build())
            } else {
                Ok(CotyledonError::InvalidPlantType(plant.get_type().to_owned()).into())
            }
        });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
