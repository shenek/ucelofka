use std::path::Path;

use crate::data::{
    entry::{Entries, Entry},
    Records,
};

pub fn list(data_path: &Path) -> Entries {
    let entry_path = data_path.join(Path::new("entries"));
    Entries::load(entry_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Option<Entry> {
    list(data_path).get(id)
}
