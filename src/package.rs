use crate::index::IndexEntry;

pub struct Package {
    name: String,
    desc: String,
    version: String,
    authors: Vec<String>,
    maintainers: Vec<String>,
    deps: Vec<Package>,
    // FIXME: Flags could become it's own type later
    flags: Vec<String>,
}

impl Package {
    pub fn from(entry: &IndexEntry) -> Self {
        unimplemented!()
    }
}
