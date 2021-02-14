use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{data_display, data_try_from, default_version};

pub const VERSION: u32 = 1;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Entry {
    #[serde(default = "default_version")]
    pub _version: u32,
    pub id: String,
    pub name: String,
    pub price: f32,
    pub currency: String,
    pub details: Vec<String>,
}

impl Entry {
    pub fn new(
        id: String,
        name: String,
        price: f32,
        currency: String,
        details: Vec<String>,
    ) -> Self {
        Self {
            _version: VERSION,
            id,
            name,
            price,
            currency,
            details,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Entries {
    pub entries: Vec<Entry>,
}

data_display!(Entry);
data_try_from!(Entry);
data_display!(Entries);
