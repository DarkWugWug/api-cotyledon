use crate::error::CotyledonError;
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

pub const PLANT_TYPES: [&str; 1] = ["carrot"];

#[derive(Deserialize, Serialize, Hash)]
pub struct Plant {
    pub plant_type: String,
    pub planted: u64,
}

impl Plant {
    pub fn new(plant_type: String) -> Result<Self, CotyledonError> {
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
pub struct NatureApproved {
    pub nature_approved: Option<String>,
    pub plants: Vec<Plant>,
}

impl NatureApproved {
    /// If the request has a top-level field of `.garden` this will enforce a
    /// top-level field of `.shadow_garden`. The shadow garden is a Base64 string of
    /// a big-endian u64 and represents the identity of this garden the last time
    /// this server processed it. This identity must be consistent with the identity
    /// of the garden in the `.garden` field. If it is not the request is rejected
    /// because the garden has been externally modified.
    ///
    /// If the request has no top-level field of `.garden` this does nothing. Furthermore,
    /// meaning the extension will be `None`.
    ///
    /// #### Note
    /// This will set the request extension to true or false if there is a `.garden`
    /// at the top-level. That value can be gotten in endpoint functions by calling
    /// `request.get_ext()`.
    pub fn is_honest(&self, expected: &str, secret: &str) -> Result<bool, CotyledonError> {
        if let Some(identity) = self.nature_approved {
            let mut hasher = DefaultHasher::new();
            plants.hash(&mut hasher);
            secret.hash(&mut hasher);
            let garden_hash = hasher.finish();
            let vec = Base64::decode_vec(&identity)
                .map_err(|e| CotyledonError::InternalError(format!("{}", e)))?;
            let bytes = match vec.try_into() {
                Ok(x) => x,
                Err(_) => return Ok(false) // The identity is not of expected form; thus, it's not honest
            };
            let expected_hash = u64::from_be_bytes(bytes);
            Ok(expected_hash == garden_hash)
        } else {
            Ok(false)
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        return Field { 
            seal_of_honesty: Option::None,
            plants: Vec::new()
        };
    }
}
