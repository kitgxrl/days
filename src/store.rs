use crate::error::StoreError;
use crate::package::Package;
use std::path::Path;

pub struct Store<'a> {
    pub directory: &'a Path,
}

impl<'a> Store<'a> {
    pub fn new(self, directory: &'a Path) -> Result<Self, StoreError> {
        if let Ok(existence) = directory.try_exists() {
            if !existence || !directory.is_dir() {
                return Err(StoreError::InvalidDirectory);
            }
        } else {
            return Err(StoreError::Other(String::from(
                "Unable to check if store directory exists",
            )))
        }

        self.check_validity();

        Ok(Self { directory })
    }

    // TODO: implement checks to verify store validity
    fn check_validity(self) -> bool {
        true  
    }

    pub fn search(name: String, version: Option<String>) -> Option<Package> {
        unimplemented!()
    }
}
