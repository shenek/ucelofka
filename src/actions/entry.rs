use anyhow::{anyhow, Result};
use std::path::Path;

use crate::data::{
    entry::{Entries, Entry},
    Record, Records,
};

pub fn list(data_path: &Path) -> Result<Entries> {
    let entry_path = data_path.join(Path::new("entries"));
    Ok(Entries::load(entry_path.as_path())?)
}

pub fn get(data_path: &Path, id: &str) -> Result<Entry> {
    Ok(list(data_path)?
        .get(id)
        .ok_or_else(|| anyhow!("Entry {} not found.", id))?)
}

pub fn create(
    data_path: &Path,
    id: String,
    name: String,
    price: f32,
    currency: String,
    details: Vec<String>,
) -> Result<Entry> {
    let entry_path = data_path.join(Path::new("entries"));
    let new_entry = Entry::new(id, name, price, currency, details);

    new_entry
        .store(&entry_path)
        .map_err(|err| anyhow!("{}", err))?;

    Ok(new_entry)
}
