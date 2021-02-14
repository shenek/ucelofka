use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{data_display, data_try_from, default_version};

pub const VERSION: u32 = 1;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identification {
    pub tax: String,
    pub registration: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identity {
    #[serde(default = "default_version")]
    pub _version: u32,
    pub id: String,
    pub name: String,
    pub address: Vec<String>,
    pub phone: Vec<String>,
    pub email: Vec<String>,
    pub www: Vec<String>,
    pub identification: Identification,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identities {
    pub identities: Vec<Identity>,
}

data_display!(Identity);
data_try_from!(Identity);
data_display!(Identities);
