use dotenv::dotenv;
use fastrand;
use serde::{Deserialize, Serialize};
use std::env::{self, VarError};
use std::time::SystemTime;
use tide::prelude::*; // Pulls in the json! macro.
use tide::{Body, Request, Response, StatusCode};

mod error;
use error::CotyledonError;

const ENV_SECRET_KEY: &str = "COTYLEDON_SECRET";

const PLANT_TYPES: [&str; 1] = ["carrot"];

#[derive(Deserialize, Serialize)]
struct Plant {
    plant_type: String,
    planted: u64,
}

impl Plant {
    fn new(plant_type: String) -> Result<Self, CotyledonError> {
        use CotyledonError::{InternalError, InvalidPlantType};
        if !PLANT_TYPES.contains(&plant_type.as_str()) {
            return Err(InvalidPlantType(format!(
                "`{}` is not a valid plant type",
                plant_type
            )));
        }
        let planted = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(x) => x.as_secs(),
            Err(e) => return Err(InternalError(format!("Time drift detected: {}", e))),
        };
        Ok(Plant {
            plant_type,
            planted,
        })
    }
}

#[derive(Deserialize, Serialize)]
struct Garden {
    plants: Vec<Plant>,
}

impl Default for Garden {
    fn default() -> Self {
        return Garden { plants: Vec::new() };
    }
}

#[derive(Deserialize, Serialize)]
struct Sow {
    plant_type: String,
    garden: Option<Garden>,
}

fn generateSeed() -> String {
    let secret = std::iter::repeat_with(fastrand::alphanumeric)
        .take(24)
        .collect();
    println!("Generating temporary value...");
    println!("{}={}", ENV_SECRET_KEY, secret);
    println!("To keep this secret run:");
    println!("\techo {}={} >> .env", ENV_SECRET_KEY, secret);
    return secret;
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv();
    let secret = match env::var(ENV_SECRET_KEY) {
        Ok(x) => x,
        Err(VarError::NotPresent) => generateSeed(),
        Err(e) => panic!("COTYLEDON_SEED found but was malformed! {}", e),
    };
    tide::log::start();
    let mut app = tide::new();

    app.at("/sow").post(|mut req: Request<()>| async move {
        let sow_action: Sow = req.body_json().await?;

        let mut garden = sow_action.garden.unwrap_or_default();
        let plant = match Plant::new(sow_action.plant_type) {
            Ok(x) => x,
            Err(e) => return Err(e.into()),
        };
        garden.plants.push(plant);

        Body::from_json(&garden)
    });

    app.at("/plants")
        .get(|_| async { Ok(json!({ "plants": PLANT_TYPES })) });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
