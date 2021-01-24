use anyhow::{anyhow, Result};
use std::path::Path;

use crate::{
    data::identity::{Identities, Identity},
    storage::Records,
};

pub fn ids(data_path: &Path) -> Result<String> {
    let data = list(data_path)?;
    Ok(data.ids().join("\n"))
}

pub fn list(data_path: &Path) -> Result<Identities> {
    let identity_path = data_path.join(Path::new("identities"));
    Ok(Identities::load(identity_path.as_path())?)
}

pub fn get(data_path: &Path, id: &str) -> Result<Identity> {
    Ok(list(data_path)?
        .get(id)
        .ok_or_else(|| anyhow!("Identity {} not found.", id))?)
}
