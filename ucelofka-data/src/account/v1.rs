#![allow(non_snake_case)]

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{data_display, data_try_from, default_version};

pub const VERSION: u32 = 1;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Account {
    #[serde(default = "default_version")]
    pub _version: u32,
    pub id: String,
    pub name: String,
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
    pub IBAN: String,
    pub BIC: String,
    pub currency: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

data_display!(Account);
data_try_from!(Account);
data_display!(Accounts);
