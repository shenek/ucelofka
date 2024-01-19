use anyhow::{anyhow, Result};
use std::path::Path;

use crate::{
    data::account::{Account, Accounts},
    storage::Records,
};

pub fn ids(data_path: &Path) -> Result<String> {
    let data = list(data_path)?;
    Ok(data.ids().join("\n"))
}

pub fn list(data_path: &Path) -> Result<Accounts> {
    let account_path = data_path.join(Path::new("accounts"));
    Accounts::load(account_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Result<Account> {
    list(data_path)?
        .get(id)
        .ok_or_else(|| anyhow!("Account {} not found.", id))
}
