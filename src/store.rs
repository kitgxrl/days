use crate::error::StoreError;
use crate::index::{Index, IndexEntry};
use crate::package::Package;
use std::path::Path;

pub struct Store<'a> {
    pub directory: &'a Path,
}

impl<'a> Store<'a> {
    pub fn new(directory: &'a Path) -> Result<Self, StoreError> {
        if let Ok(existence) = directory.try_exists() {
            if !existence || !directory.is_dir() {
                return Err(StoreError::InvalidDirectory);
            }
        } else {
            return Err(StoreError::Other(String::from(
                "Unable to check if store directory exists",
            )));
        }

        Store::check_validity(directory);

        Ok(Self { directory })
    }

    // TODO: implement checks to verify store validity
    fn check_validity(path: &'a Path) -> bool {
        true
    }

    pub fn search(
        self,
        name: String,
        category: Option<String>,
        version: Option<String>,
    ) -> Option<Package> {
        let index = Index::from(self);

        let filtered = index
            .entries
            .iter()
            .filter(|pkg| {
                let to_search = pkg.name == name;

                if let Some(cat) = &category {
                    let to_search = to_search && &pkg.category == cat;
                }
                if let Some(ver) = &version {
                    let to_search = to_search && &pkg.version == ver;
                }

                to_search
            })
            .collect::<Vec<&IndexEntry>>();

        match filtered.len() {
            1 => Some(Package::from(filtered[0])),
            2.. => unimplemented!(), // TODO: Handle ambigious package names
            _ => None,
        }
    }
}
