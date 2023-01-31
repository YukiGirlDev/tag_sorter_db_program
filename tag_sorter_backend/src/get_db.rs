use thiserror::Error;

use crate::{
    get_settings::{get_settings, SettingsError},
    item::Item,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Getting db but: {0}")]
    Settings(#[from] SettingsError),
    #[error("Failed to read the database. (OS error: {0})")]
    FailedTOReadDb(#[from] std::io::Error),
    #[error("When grabing the database failed to deserialise file. It may be corrupt. (Serde error: {0})")]
    FailedToDeserialise(#[from] serde_json::Error)
}

pub async fn get_db() -> Result<Vec<Item>, Error> {
    let settings = get_settings().await;
    let path = settings?;
    let data = rocket::tokio::fs::read_to_string(path.join("db.json")).await?;
    serde_json::from_str(&data).map_err(std::convert::Into::into)
}
