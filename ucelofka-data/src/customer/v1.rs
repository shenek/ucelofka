use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{data_display, data_try_from, default_version};

pub const VERSION: u32 = 1;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Customer {
    #[serde(default = "default_version")]
    pub _version: u32,
    pub id: String,
    pub name: String,
    pub address: Vec<String>,
    pub identification: String,
    pub email: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Customers {
    pub customers: Vec<Customer>,
}

data_display!(Customer);
data_try_from!(Customer);
data_display!(Customers);
