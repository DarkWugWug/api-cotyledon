use crate::error::{CotyledonError, SignatureError};
use crate::plants::Plant;
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

#[derive(Deserialize, Serialize, Clone)]
pub struct NatureApproved {
    nature_approved: Option<String>,
    plot: Plot,
}

impl NatureApproved {
    pub fn new() -> Self {
        let plot = Plot::new();
        NatureApproved {
            nature_approved: None,
            plot,
        }
    }

    pub fn get_plot(&self) -> &Plot {
        &self.plot
    }

    pub fn get_mut_plot(&mut self) -> &mut Plot {
        &mut self.plot
    }

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
    pub fn is_honest(&self, secret: &str) -> Result<(), CotyledonError> {
        let plot_hash = self.get_plot_hash(secret);
        let expected_hash = self.get_signature()?;
        if expected_hash != plot_hash {
            return Err(CotyledonError::InvalidSignature(SignatureError::Mismatch));
        }
        Ok(())
    }

    fn get_signature(&self) -> Result<u64, CotyledonError> {
        if let Some(expected_raw) = &self.nature_approved {
            let vec = Base64::decode_vec(&expected_raw)
                .map_err(|e| CotyledonError::InternalError(format!("{}", e)))?;
            let bytes = vec
                .try_into()
                .map_err(|_| CotyledonError::InvalidSignature(SignatureError::Malformed))?;
            Ok(u64::from_be_bytes(bytes))
        } else {
            Err(CotyledonError::InvalidSignature(SignatureError::NotFound))
        }
    }

    fn get_plot_hash(&self, secret: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.plot.hash(&mut hasher);
        secret.hash(&mut hasher);
        return hasher.finish();
    }

    pub fn sign(&mut self, secret: &str) {
        let signature = self.get_plot_hash(secret);
        self.nature_approved = Some(Base64::encode_string(&signature.to_be_bytes()));
    }
}

#[derive(Deserialize, Serialize, Hash, Clone)]
pub struct Plot {
    plants: Vec<Plant>,
}

impl Plot {
    pub fn new() -> Self {
        Plot { plants: Vec::new() }
    }

    pub fn get_plants(&self) -> &[Plant] {
        &self.plants
    }

    pub fn get_mut_plants(&mut self) -> &mut Vec<Plant> {
        &mut self.plants
    }
}

#[derive(Deserialize, Serialize, Hash, Clone, Copy, Debug)]
pub struct SimpleDuration(u64);

impl SimpleDuration {
    pub fn as_secs(&self) -> u64 {
        self.0
    }
}

impl From<Duration> for SimpleDuration {
    fn from(dur: Duration) -> Self {
        SimpleDuration(dur.as_secs())
    }
}

impl From<SimpleDuration> for Duration {
    fn from(x: SimpleDuration) -> Duration {
        let SimpleDuration(secs) = x;
        Duration::from_secs(secs)
    }
}