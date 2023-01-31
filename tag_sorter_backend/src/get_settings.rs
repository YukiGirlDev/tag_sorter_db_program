use std::path::PathBuf;

use thiserror::Error;

use crate::SETTINGSPATH;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct SettingsError(String);

pub async fn get_settings() -> Result<PathBuf, SettingsError> {
    rocket::tokio::fs::read_to_string(SETTINGSPATH)
        .await
        .map_err(|err| SettingsError(err.to_string()))
        .map(PathBuf::from)
}