use crate::error::CotyledonError;
use crate::plants::Plant;
use crate::infra::NatureApproved;
use crate::GardenSecrets;
use serde::{Deserialize, Serialize};
use tide::{Response, Request, StatusCode };
use tide::prelude::*;

impl NatureApproved {
    fn sow(&mut self, plant: Plant) {
        self.get_mut_plot().get_mut_plants().push(plant);
    }
}

#[derive(Deserialize, Serialize)]
struct Sow {
    plant_type: String,
    garden: NatureApproved,
}

pub async fn sow_plant(mut req: Request<GardenSecrets>) -> tide::Result {
    let mut sow: Sow = req.body_json().await?;
    if !sow.garden.get_plot().get_plants().is_empty() {
        match sow.garden.is_honest(&req.state().secret) {
            Ok(_) => (),
            Err(err) => return Ok(err.into())
        };
    }
    let plant = match Plant::new(sow.plant_type) {
        Ok(x) => x,
        Err(e) if matches!(CotyledonError::InvalidPlantType, _e) => return Ok(e.into()),
        Err(e) => return Err(e.into())
    };
    sow.garden.get_mut_plot().get_mut_plants().push(plant);
    sow.garden.sign(&req.state().secret);
    Ok(Response::builder(StatusCode::Ok).body(json!(sow.garden)).build())
}
