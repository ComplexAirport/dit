use crate::helpers::read_to_string;
use crate::errors::DitResult;
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;
use std::fs;

/// Implements methods for serializing structures to files and
/// deserializing from files
pub trait DitModel: Sized {
    fn serialize_to(&self, path: &Path) -> DitResult<()>;
    fn deserialize_from(path: &Path) -> DitResult<Self>;
}

impl<T> DitModel for T where T: Serialize + DeserializeOwned  {
    fn serialize_to(&self, path: &Path) -> DitResult<()> {
        let serialized = serde_json::to_string_pretty(self)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    fn deserialize_from(path: &Path) -> DitResult<Self> {
        let serialized = read_to_string(path)?;
        Ok(serde_json::from_str(&serialized)?)
    }
}


/// Implements methods for serializing structures to files and
/// deserializing from files. Deserializes into default values
/// if the content is not found
pub trait DitModelDefault: Sized {
    fn deserialize_default_from(path: &Path) -> DitResult<Self>;
}

impl<T> DitModelDefault for T where T: Default + DeserializeOwned {
    fn deserialize_default_from(path: &Path) -> DitResult<Self> {
        let serialized = read_to_string(path)?;
        if serialized.is_empty() {
            Ok(Default::default())
        } else {
            Ok(serde_json::from_str(&serialized)?)
        }
    }
}
