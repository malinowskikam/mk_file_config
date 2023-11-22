use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{read_to_string, write},
    path::Path,
};

use crate::FileConfigError;

pub trait FileConfig: Clone + Default + Serialize + DeserializeOwned {
    fn load(config_path: &Path) -> Result<Self, FileConfigError> {
        let config_yaml = read_to_string(config_path)?;
        Ok(serde_yaml::from_str::<Self>(&config_yaml)?)
    }

    fn save(&self, config_path: &Path) -> Result<(), FileConfigError> {
        let config_yaml = serde_yaml::to_string(&self)?;
        Ok(write(config_path, config_yaml)?)
    }

    fn init() -> Result<Self, FileConfigError> {
        let config_path = Path::new("config.yml");
        let config: Self;
        if config_path.exists() {
            config = Self::load(config_path)?
        } else {
            config = Self::default();
            config.save(config_path)?
        };
        Ok(config)
    }
}
