use serde::{Deserialize, Serialize};
use tide::Request;
use crate::infra::Field;

#[derive(Deserialize, Serialize)]
struct Sow {
    plant_type: String,
    garden: Field,
}

pub async fn sow_plant(req: Request<GardenSecrets>) -> tide::Result {
    let sow_action: Sow = req.body_json().await?;

    let mut garden = sow_action.garden.unwrap_or_default();
    let plant = match Plant::new(sow_action.plant_type) {
        Ok(x) => x,
        Err(e) => return Err(e.into()),
    };
    garden.plants.push(plant);

    Body::from_json(&garden)
}