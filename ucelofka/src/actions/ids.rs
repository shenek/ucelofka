use anyhow::Result;
use std::path::Path;

use super::{account, customer, entry, identity, invoice, template};

pub fn ids(data_path: &Path) -> Result<String> {
    let mut result = String::new();

    result += "# Identities:\n";
    result += &identity::ids(data_path)?;

    result += "\n\n# Accounts:\n";
    result += &account::ids(data_path)?;

    result += "\n\n# Customers:\n";
    result += &customer::ids(data_path)?;

    result += "\n\n# Entries:\n";
    result += &entry::ids(data_path)?;

    result += "\n\n# Invoices:\n";
    result += &invoice::ids(data_path)?;

    result += "\n\n# Templates:\n";
    result += &template::list(data_path)?
        .templates
        .iter()
        .map(|template| template.name.clone() + "\n")
        .collect::<String>();

    Ok(result)
}
