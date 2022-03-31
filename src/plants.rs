use crate::error::CotyledonError;
use crate::infra::SimpleDuration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

lazy_static! {
    pub static ref COTYLEDON_PLANTS: HashMap<&'static str, CotyledonPlant> = {
        let mut m = HashMap::new();
        m.insert(
            "carrot",
            CotyledonPlant {
                grow_time: Duration::from_secs(86_400).into(),
            },
        );
        m.insert(
            "potato",
            CotyledonPlant {
                grow_time: Duration::from_secs(60).into(),
            },
        );
        m.insert(
            "onion",
            CotyledonPlant {
                grow_time: Duration::from_secs(3_600).into(),
            },
        );
        m
    };
}

#[derive(Deserialize, Serialize)]
pub struct CotyledonPlant {
    grow_time: SimpleDuration,
}

impl CotyledonPlant {
    #[inline]
    pub fn is_mature<T>(&self, dur: T) -> bool 
        where T: Into<Duration>
    {
        dur.into().as_secs() > self.grow_time.as_secs()
    }
}

#[derive(Deserialize, Serialize, Hash, Clone)]
pub struct Plant {
    plant_type: String,
    planted: SimpleDuration,
}

impl Plant {
    pub fn new(plant_type: String) -> Result<Self, CotyledonError> {
        use CotyledonError::{InternalError, InvalidPlantType};
        if !COTYLEDON_PLANTS.contains_key(&plant_type.as_str()) {
            return Err(InvalidPlantType(format!(
                "`{}` is not a valid plant type",
                plant_type
            )));
        }
        let planted = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(x) => x.into(),
            Err(e) => return Err(InternalError(format!("Time drift detected: {}", e))),
        };
        Ok(Plant {
            plant_type,
            planted,
        })
    }

    pub fn get_type(&self) -> &str {
        &self.plant_type
    }

    pub fn elapsed(&self) -> Result<Duration, CotyledonError> {
        use CotyledonError::InternalError;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|err| {
                InternalError(format!(
                    "The timestamp given is in the future! Panic: {}",
                    err
                ))
            })
            .and_then(|x| {
                x.checked_sub(self.planted.into())
                    .ok_or(InternalError(format!(
                        "The timestamp for this plant is in the future!"
                    )))
            })
    }
}
