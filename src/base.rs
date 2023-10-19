use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs::{read_to_string, write},
    path::Path,
};

pub trait FileConfig: Clone + Default + Serialize + DeserializeOwned {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfigBase<T: FileConfig> {
    #[serde(bound(deserialize = ""))]
    pub config: T,
}

impl<T: FileConfig> FileConfigBase<T> {
    fn load(config_path: &Path) -> Result<Self> {
        let config_yaml = read_to_string(config_path)?;
        Ok(serde_yaml::from_str::<Self>(&config_yaml)?)
    }

    fn save(&self, config_path: &Path) -> Result<()> {
        let config_yaml = serde_yaml::to_string(&self)?;
        Ok(write(config_path, config_yaml)?)
    }

    pub fn init() -> Result<Self> {
        let config_path = Path::new("config.yml");
        let config: Self;
        if config_path.exists() {
            println!("Loading existing config from config.yml");
            config = Self::load(&config_path)?
        } else {
            println!("Config file not found. Saving default config in config.yml");
            config = Self::default();
            config.save(&config_path)?
        };
        Ok(config)
    }
}

impl<T: FileConfig> Default for FileConfigBase<T> {
    fn default() -> Self {
        Self {
            config: T::default(),
        }
    }
}