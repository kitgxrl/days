use crate::package::Package;
use crate::store::Store;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::slice::SliceIndex;

#[derive(Debug)]
pub struct IndexEntry {
    pub name: String,
    pub category: String,
    pub version: String,
}

lazy_static! {
    static ref SPLIT_REGEX: Regex = Regex::new(r"[\/\ ]").unwrap();
}

pub fn parse(store: &Store) -> Vec<IndexEntry> {
    let index = store.directory.join("pkgs.cnt");

    // Let's read the file first

    let mut file = File::open(index).expect("Failed to open index");
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    // Convert a given line to an IndexEntry

    let entries: Vec<IndexEntry> = contents.lines().map(parse_line).collect();

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

pub fn search(
    index: Vec<IndexEntry>,
    name: String,
    category: Option<String>,
    version: Option<String>,
) -> Option<Package> {
    let filtered = index
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
