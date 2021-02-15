use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{data_display, data_try_from, default_version};

use super::v1;
use crate::identification::Identification;

pub const VERSION: u32 = 2;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Customer {
    #[serde(default = "default_version")]
    pub _version: u32,
    pub id: String,
    pub name: String,
    pub address: Vec<String>,
    pub identifications: Vec<Identification>,
    pub email: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Customers {
    pub customers: Vec<Customer>,
}

impl From<v1::Customer> for Customer {
    fn from(old: v1::Customer) -> Self {
        let identifications = old
            .identification
            .splitn(2, '/')
            .map(|e| e.trim())
            .zip(["registration", "tax"].iter())
            .map(|(value, name)| Identification {
                name: name.to_string(),
                value: value.to_string(),
            })
            .collect();
        Self {
            _version: VERSION,
            id: old.id,
            name: old.name,
            address: old.address,
            email: old.email,
            identifications,
        }
    }
}

data_display!(Customer);
data_try_from!(Customer);
data_display!(Customers);
