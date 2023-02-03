use std::collections::HashSet;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{get_settings::get_settings, item::Item};

use self::filter::{Filter, FilterDate};

const APIVERSION: &str = "0.1.0";

pub mod filter;

#[cfg(test)]
mod test;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Version mismatch the server is running version {APIVERSION} and the backend is running version {0}")]
    IncorrectVersion(String),
    #[error("{0}")]
    Editing(#[from] crate::edit::Error),
    #[error("{0}")]
    Settings(#[from] crate::get_settings::SettingsError),
    #[error("Failed IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to deserialise the database (Serde error {0})")]
    Serde(#[from] serde_json::Error),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Action {
    Create(Item),
    Edit(Item),
    Delete(Filter),
    SetDate(Filter, FilterDate),
    AddTag(Filter, String),
    ReplaceTag(Filter, String, String),
    RemoveTag(Filter, String),
    SetTags(Filter, HashSet<String>),
}
#[derive(Deserialize, Serialize)]
pub struct DataBaseAction {
    pub version: String,
    pub action: Action,
}

#[allow(clippy::todo)]
impl DataBaseAction {
    pub async fn run(self, mut db: Vec<Item>) -> Result<Vec<Item>, Error> {
        // Check api version
        if self.version != APIVERSION {
            return Err(Error::IncorrectVersion(self.version));
        }
        // Run action
        match self.action {
            Action::Create(item) => {
                db.push(item);
                Ok(db)
            }
            Action::Edit(edit) => crate::edit::run(edit, db)
                .await
                .map_err(std::convert::Into::into),
            Action::Delete(filter) => {
                let db_path = get_settings().await?;
                let items = filter.run(&mut db);
                let mut ids = Vec::new();
                for i in items {
                    ids.push(i.id);
                    rocket::tokio::fs::remove_file(db_path.join(&i.path)).await?;
                }
                Ok(db.into_iter().filter(|x| !ids.contains(&x.id)).collect())
            }
            Action::SetDate(filter, date) => {
                let items = filter.run(&mut db);
                for i in items {
                    i.year = date.year;
                    i.month = date.month;
                    i.day = date.day;
                }
                Ok(db)
            }
            Action::AddTag(filter, tag) => {
                let items = filter.run(&mut db);
                for i in items {
                    i.tags.insert(tag.clone());
                }
                Ok(db)
            }
            Action::ReplaceTag(filter, oldtag, newtag) => {
                let items = filter.run(&mut db);
                for i in items {
                    if i.tags.contains(&oldtag) {
                        i.tags.remove(&oldtag);
                        i.tags.insert(newtag.clone());
                    }
                }
                Ok(db)
            }
            Action::RemoveTag(filter, tag) => {
                let items = filter.run(&mut db);
                for i in items {
                    i.tags.remove(&tag);
                }
                Ok(db)
            }
            Action::SetTags(filter, tags) => {
                let items = filter.run(&mut db);
                for i in items {
                    i.tags = tags.clone();
                }
                Ok(db)
            }
        }
    }
}

#[post("/action", format = "application/json", data = "<action>")]
pub async fn action(action: Json<DataBaseAction>) -> String {
    match run_action(action.0).await {
        Ok(_) => String::from(r#"{"is_ok":true}"#),
        Err(err) => format!("{{\"is_ok\":false,\"error\":{:?}}}", err.to_string()),
    }
}

async fn run_action(action: DataBaseAction) -> Result<(), Error> {
    // Read the database
    let db_path = get_settings().await?.join("db.json");
    let db = rocket::tokio::fs::read_to_string(&db_path).await?;
    let db = serde_json::from_str(&db)?;
    // Run the aciton
    let new_db = action.run(db).await?;
    // Write back to the database
    let new_db_string = serde_json::to_string(&new_db)?;
    rocket::tokio::fs::write(db_path, new_db_string.as_bytes()).await?;
    Ok(())
}
