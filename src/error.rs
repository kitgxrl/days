use std::fmt;

#[derive(Debug)]
pub enum StoreError {
    NotFound(String),
    InvalidDirectory,
    Other(String)
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::NotFound(pkg) => f.write_str(&format!("The package `{}` was unable to be found.", pkg)),
            StoreError::InvalidDirectory => f.write_str("Store directory does not exist or is not a directory"),
            StoreError::Other(msg) => f.write_str(msg)
        }
    }
}
