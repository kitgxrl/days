use crate::error::StoreError;
use crate::index::{self, IndexEntry};
use crate::package::Package;
use std::path::Path;

pub struct Store<'a> {
    pub directory: &'a Path,
}

pub fn create_store<'a>(directory: &'a Path) -> Result<Store, StoreError> {
    check_validity(directory)?;

    Ok(Store { directory })
}

fn check_validity<'a>(directory: &'a Path) -> Result<(), StoreError> {
    if let Ok(existence) = directory.try_exists() {
        if !existence || !directory.is_dir() {
            return Err(StoreError::InvalidDirectory);
        }
    } else {
        return Err(StoreError::Other(String::from(
            "Unable to check if store directory exists",
        )));
    }

    Ok(())
}
