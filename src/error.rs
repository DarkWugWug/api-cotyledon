use std::{error::Error, fmt};
use tide::prelude::*;
use tide::{Response, StatusCode}; // Pulls in the json! macro.

#[derive(Debug)]
pub enum SignatureError {
    NotFound,
    Malformed,
    Mismatch,
}

impl fmt::Display for SignatureError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use SignatureError::*;
        match self {
            NotFound => write!(fmt, "Expected signature was missing. Do not modify or omit the `nature_approved` field."),
            Malformed => write!(fmt, "Signature was found, but not in the proper form. Has it been modified?"),
            Mismatch => write!(fmt, "Signature was invalid. Have the plants been modified? Do not modify them; nature is always right.")
        }
    }
}

#[derive(Debug)]
pub enum CotyledonError {
    InvalidSignature(SignatureError),
    InvalidPlantType(String),
    InternalError(String),
}

impl Error for CotyledonError {}

impl fmt::Display for CotyledonError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use CotyledonError::*;
        match self {
            InvalidSignature(x) => write!(fmt, "{}", x),
            InvalidPlantType(x) => write!(fmt, "Invalid plant type, '{}'", x),
            InternalError(x) => write!(fmt, "InternalError, {}", x),
        }
    }
}

impl From<CotyledonError> for Response {
    fn from(x: CotyledonError) -> Response {
        use CotyledonError::*;
        match x {
            InvalidSignature(x) => Response::builder(StatusCode::Forbidden)
                .body(json!({ "error": format!("{}", x) }))
                .build(),
            InvalidPlantType(x) => Response::builder(StatusCode::BadRequest)
                .body(json!({ "error": format!("{}", x) }))
                .into(),
            InternalError(x) => {
                eprintln!("InternalError: {}", x);
                Response::new(StatusCode::InternalServerError)
            }
        }
    }
}
