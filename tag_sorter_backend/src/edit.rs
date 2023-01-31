use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::{
    get_settings::{get_settings, SettingsError},
    item::Item,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Failed to read the settings file! It may have been deleted or moved! (OS error: {0})"
    )]
    GetSettings(#[from] SettingsError),
    #[error("{0}")]
    Rename(#[from] RenameError),
    #[error("The item requested for edit (with id of {0}) does not exist.")]
    RequestedItemDoesNotExist(usize),
}

/// Performs the edit operation
pub async fn run(new: Item, mut items: Vec<Item>) -> Result<Vec<Item>, Error> {
    // Get database file location from settings
    let settings = get_settings().await?;

    // Find the item being edited
    let mut editing_name = false;
    let mut edited_item = None;
    for (i, item) in items.iter().enumerate() {
        if new.id == item.id {
            // If the names do not match set flag for renaming it
            editing_name = new.path != item.path;
            edited_item = Some(i);
            break;
        }
    }
    // i is index in items of the item being edited
    let i = edited_item.ok_or(Error::RequestedItemDoesNotExist(new.id))?;

    // Rename file if needed
    if editing_name {
        rename(&settings, &new.path, &items[i].path).await?;
    }
    // Edit the item
    items[i] = new;
    // Return modified database
    Ok(items)
}

#[derive(Debug, Error)]
pub enum RenameError {
    #[error("The path {} already exists. The old path {} will be used insted.", .0.display(), .1.display())]
    PathAlreadyExists(PathBuf, PathBuf),
    #[error("The directory {} has not parent. This means the filepath must have been using directory traveral operations this is forbiden.", .0.display())]
    NoParent(PathBuf),
    #[error("The parent directory ({}) of the file was not readable (OS error: {})this could mean that the directory was restricted and the user running the database can not access it.", .0.display(), .1)]
    FailedToReadParentPath(PathBuf, String),
    #[error("Failed to rename '{}' to '{}' (os error: {2})", .0.display(), .1.display())]
    FailedToRename(PathBuf, PathBuf, String),
}
/// Rename the file at ``old_path`` to ``new_path``
/// All paths are relative to ``settings``
/// This function is only called by edit
async fn rename(
    settings: &Path,
    new_path: &PathBuf,
    old_path: &PathBuf,
) -> Result<(), RenameError> {
    let parent_path = PathBuf::from(
        settings
            .join(new_path)
            .parent()
            .ok_or_else(|| RenameError::NoParent(settings.join(new_path)))?,
    );
    let mut dirs = rocket::tokio::fs::read_dir(&parent_path)
        .await
        .map_err(|err| RenameError::FailedToReadParentPath(parent_path, err.to_string()))?;
    let mut current = dirs.next_entry().await;

    // Check the parent_path directory for a duplicate file
    let mut duplicate = false;
    while let Ok(Some(item)) = &current {
        if Some(item.file_name().as_os_str()) == new_path.file_name() {
            duplicate = true;
        }
        current = dirs.next_entry().await;
    }
    // If there is a duplicate return an error else rename the file and return ok
    if duplicate {
        Err(RenameError::PathAlreadyExists(
            new_path.clone(),
            old_path.clone(),
        ))
    } else {
        let from = settings.join(old_path);
        let to = settings.join(new_path);
        rocket::tokio::fs::rename(&from, &to)
            .await
            .map_err(|err| RenameError::FailedToRename(from, to, err.to_string()))
    }
}
