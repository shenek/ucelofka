use anyhow::{anyhow, Result};
use git2::Repository;
use std::path::Path;

use crate::{
    data::entry::{Entries, Entry},
    storage::{Record, Records},
};

pub fn ids(data_path: &Path) -> Result<String> {
    let data = list(data_path)?;
    Ok(data.ids().join("\n"))
}

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
    git: bool,
) -> Result<Entry> {
    let entry_path = data_path.join(Path::new("entries"));
    let new_entry = Entry::new(id, name, price, currency, details);

    let mut repository = if git {
        Some(
            Repository::open(data_path)
                .map_err(|err| anyhow!("Faield to open git repository {}", err))?,
        )
    } else {
        None
    };

    new_entry
        .store(&entry_path)
        .map_err(|err| anyhow!("{}", err))?;

    if let Some(repo) = repository.as_mut() {
        let new_path = Path::new("entries").join(new_entry.filename());

        let mut index = repo
            .index()
            .map_err(|err| anyhow!("Failed to get repo index ({})", err))?;
        index.add_path(&new_path).map_err(|err| {
            anyhow!(
                "Failed to add a file {} ({})",
                new_path.to_string_lossy(),
                err
            )
        })?;

        index
            .write()
            .map_err(|err| anyhow!("Failed to write to index ({})", err))?;
    }

    Ok(new_entry)
}
