use std::{error::Error, fmt};
use tide::{Response, StatusCode};

#[derive(Debug)]
pub enum CotyledonError {
    InvalidPlantType(String),
    InternalError(String),
}

impl Error for CotyledonError {}

impl fmt::Display for CotyledonError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use CotyledonError::*;
        match self {
            InvalidPlantType(x) => write!(fmt, "Invalid plant type, '{}'", x),
            InternalError(x) => write!(fmt, "InternalError, {}", x),
        }
    }
}

impl From<CotyledonError> for Response {
    fn from(x: CotyledonError) -> Response {
        use CotyledonError::*;
        match x {
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
