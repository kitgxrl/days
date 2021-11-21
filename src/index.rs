use crate::store::Store;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::slice::SliceIndex;

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

lazy_static! {
    static ref SPLIT_REGEX: Regex = Regex::new(r"[\/\ ]").unwrap();
}

impl<'a> Index<'a> {
    pub fn from(store: Store<'a>) -> Self {
        let entries = Index::parse(&store);

        Self { store, entries }
    }

    fn parse(store: &Store) -> Vec<IndexEntry> {
        let index = store.directory.join("pkgs.cnt");

        // Let's read the file first

        let mut file = File::open(index).expect("Failed to open index");
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        // Convert a given line to an IndexEntry

        let entries: Vec<IndexEntry> = contents.lines().map(Index::parse_line).collect();

        entries
    }

    fn parse_line(line: &str) -> IndexEntry {
        let split: Vec<&str> = SPLIT_REGEX.split(&line).collect();

        let name = split.get(1);
        let category = split.get(0);
        let version = split.get(2);

        if let (Some(n), Some(c), Some(v)) = (name, category, version) {
            IndexEntry {
                name: n.to_string(),
                category: c.to_string(),
                version: v.to_string(),
            }
        } else {
            panic!("Failed to parse index")
        }
    }
}
