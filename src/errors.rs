use thiserror::Error;

#[derive(Error, Debug)]
pub enum GbRepoError {
    #[error("A user with this username was not found")]
    UserNotFound,
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    TomlSerializationError(#[from] toml::ser::Error),
    #[error(transparent)]
    JSONSerializationError(#[from] serde_json::Error),
    #[error("Invalid output format")]
    InvalidOutputFormat,
}
