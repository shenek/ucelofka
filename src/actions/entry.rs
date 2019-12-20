use failure::Fail;
use std::path::Path;

use crate::data::{
    entry::{Entries, Entry},
    Record, Records,
};

#[derive(Fail, Debug)]
#[fail(display = "Create entry failed {}", msg)]
pub struct CreateError {
    msg: String,
}

pub fn list(data_path: &Path) -> Entries {
    let entry_path = data_path.join(Path::new("entries"));
    Entries::load(entry_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Option<Entry> {
    list(data_path).get(id)
}

pub fn create(
    data_path: &Path,
    id: String,
    name: String,
    price: f32,
    currency: String,
    details: Vec<String>,
) -> Result<Entry, CreateError> {
    let entry_path = data_path.join(Path::new("entries"));
    let new_entry = Entry::new(id, name, price, currency, details);

    new_entry.store(&entry_path).map_err(|err| CreateError {
        msg: format!("{}", err),
    })?;

    Ok(new_entry)
}
