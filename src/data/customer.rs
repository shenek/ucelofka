use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};

use super::{list_directory, load_records};

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    name: String,
    address: Vec<String>,
    identification: String,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Customers {
    customers: Vec<Customer>,
}

impl Customers {
    pub fn load(customer_dir: &Path) -> Self {
        let paths = list_directory(customer_dir);
        Self {
            customers: load_records::<Customer>(paths),
        }
    }
}

impl fmt::Display for Customers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

impl TryFrom<String> for Customer {
    type Error = serde_yaml::Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Ok(serde_yaml::from_str(&input)?)
    }
}
