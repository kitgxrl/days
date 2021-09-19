use std::path::Path;

use crate::{error::StoreError, index::Index, store::Store};

#[test]
pub fn works() {
    ()
}

#[test]
pub fn parsed_index() -> Result<(), StoreError> {
    let ps = Store::new(Path::new("./test_pkgdb/"));

    let store = match ps {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let index = Index::from(store);

    // println!("{:#?}", index.entries);

    Ok(())
}
