#![allow(non_snake_case)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Accounts {
    pub accounts: Vec<Account>,
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
