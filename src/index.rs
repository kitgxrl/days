use crate::store::Store;
use std::fs::File;
use std::io::Read;

pub struct Index<'a> {
    store: Store<'a>,
    pub entries: Vec<IndexEntry>,
}

pub struct IndexEntry {
    pub name: String,
    pub version: String,
}

impl<'a> Index<'a> {
    pub fn from(store: Store<'a>) -> Self {
        Self {
            store,
            entries: Vec::new(),
        }
    }

    fn parse(&self) -> Vec<IndexEntry> {
        let mut entries = Vec::new();

        let index = self.store.directory.join("/pkgs.cnt");
        let mut file = File::open(index).expect("Failed to open index");
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let mut split = line.split(" ");

            let name = split.nth(0);
            let version = split.nth(1);

            // TODO: warn line is invalid
            // Ideally this will never happen, especially with official repos

            if name.is_none() || version.is_none() {
                continue;
            } else {
                let name = name.unwrap().to_string();
                let version = version.unwrap().to_string();
                entries.push(IndexEntry { name, version });
            }
        }

        entries
    }
}
