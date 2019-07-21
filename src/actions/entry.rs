use std::path::Path;

use crate::data::entry::{Entries, Entry};

pub fn list(data_path: &Path) -> Entries {
    let entry_path = data_path.join(Path::new("entries"));
    Entries::load(entry_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Option<Entry> {
    for entry in list(data_path).entries {
        if entry.id == id {
            return Some(entry);
        }
    }
    None
}
