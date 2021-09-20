use crate::store::Store;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

pub struct Index<'a> {
    store: Store<'a>,
    pub entries: Vec<IndexEntry>,
}

#[derive(Debug)]
pub struct IndexEntry {
    pub name: String,
    pub category: String,
    pub version: String,
}

impl<'a> Index<'a> {
    pub fn from(store: Store<'a>) -> Self {
        let entries = Index::parse(&store);

        Self { store, entries }
    }

    fn parse(store: &Store) -> Vec<IndexEntry> {
        let mut entries = Vec::new();

        let index = store.directory.join("pkgs.cnt");
        println!("{:#?}", index);
        let mut file = File::open(index).expect("Failed to open index");
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let mut line_number = 1;
        for line in contents.lines() {
            let replaced = line.replace('/', " ");
            let mut split = replaced.split(' ');

            let possible_name = split.next();
            let possible_category = split.next();
            let possible_version = split.next();

            let (name, category, version) =
                match (possible_name, possible_category, possible_version) {
                    (Some(n), Some(c), Some(v)) => (n.to_string(), c.to_string(), v.to_string()),
                    _ => {
                        debug!("Line Number `{}` in the `pkgs.cnt` is invalid", line_number);
                        continue;
                    }
                };

            entries.push(IndexEntry {
                name,
                category,
                version,
            });
            line_number += 1;
        }

        entries
    }
}
