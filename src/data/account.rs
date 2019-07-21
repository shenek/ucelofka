#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};

use super::{list_directory, load_records};

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
    pub IBAN: String,
    pub BIC: String,
    pub currency: String,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

impl Accounts {
    pub fn load(account_dir: &Path) -> Self {
        let paths = list_directory(account_dir);
        Self {
            accounts: load_records::<Account>(paths),
        }
    }
}

impl fmt::Display for Accounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

impl TryFrom<String> for Account {
    type Error = serde_yaml::Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Ok(serde_yaml::from_str(&input)?)
    }
}
