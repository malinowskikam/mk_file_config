use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileConfigError {
    #[error("Error while performing IO operation")]
    IOError(#[from] std::io::Error),
    #[error("Error while serializing/deserializing")]
    ParseError(#[from] serde_yaml::Error),
}
