#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};

use super::{Record, Records};

#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl Record for Account {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

impl<'a> Records<'a, Account> for Accounts {
    fn new(accounts: Vec<Account>) -> Self {
        Self { accounts }
    }

    fn load(dir: &Path) -> Self {
        let paths = Self::list_directory(dir);
        Self::new(Self::load_records(paths))
    }

    fn records(&'a self) -> &'a [Account] {
        &self.accounts
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
