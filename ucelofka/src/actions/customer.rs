use anyhow::{anyhow, Result};
use std::path::Path;

use crate::{
    data::customer::{Customer, Customers},
    storage::Records,
};

pub fn ids(data_path: &Path) -> Result<String> {
    let data = list(data_path)?;
    Ok(data.ids().join("\n"))
}

pub fn list(data_path: &Path) -> Result<Customers> {
    let customer_path = data_path.join(Path::new("customers"));
    Customers::load(customer_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Result<Customer> {
    list(data_path)?
        .get(id)
        .ok_or_else(|| anyhow!("Customer {} not found.", id))
}
