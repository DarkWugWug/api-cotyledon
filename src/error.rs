use std::{error::Error, fmt};
use tide::prelude::*;
use tide::{Response, StatusCode}; // Pulls in the json! macro.

#[derive(Debug)]
pub enum CotyledonError {
    IllegalGarden,
    InvalidPlantType(String),
    InternalError(String),
}

impl Error for CotyledonError {}

impl fmt::Display for CotyledonError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use CotyledonError::*;
        match self {
            IllegalGarden => write!(fmt, "This garden doesn't have a valid signature"),
            InvalidPlantType(x) => write!(fmt, "Invalid plant type, '{}'", x),
            InternalError(x) => write!(fmt, "InternalError, {}", x),
        }
    }
}

impl From<CotyledonError> for Response {
    fn from(x: CotyledonError) -> Response {
        use CotyledonError::*;
        match x {
            IllegalGarden => Response::builder(StatusCode::Forbidden)
                .body(json!(
                        { "error": "A locust swarm has ate all of your crops! Make sure you don't modify you're garden and just let nature run it's course."}
                ))
                .build(),
            InvalidPlantType(x) => Response::builder(StatusCode::BadRequest)
                .body(format!("{}", x))
                .into(),
            InternalError(x) => {
                eprintln!("InternalError: {}", x);
                Response::new(StatusCode::InternalServerError)
            }
        }
    }
}
