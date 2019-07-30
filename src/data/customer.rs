use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};

use super::{Record, Records};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub address: Vec<String>,
    pub identification: String,
    pub email: Vec<String>,
}

impl Record for Customer {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Customers {
    pub customers: Vec<Customer>,
}

impl<'a> Records<'a, Customer> for Customers {
    fn new(customers: Vec<Customer>) -> Self {
        Self { customers }
    }

    fn load(dir: &Path) -> Self {
        let paths = Self::list_directory(dir);
        Self::new(Self::load_records(paths))
    }

    fn records(&'a self) -> &'a [Customer] {
        &self.customers
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
