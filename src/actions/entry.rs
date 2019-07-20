use std::path::Path;

use crate::data::entry::Entries;

pub fn list(data_path: &Path) {
    let entry_path = data_path.join(Path::new("entries"));
    println!("{}", Entries::load(entry_path.as_path()));
}
