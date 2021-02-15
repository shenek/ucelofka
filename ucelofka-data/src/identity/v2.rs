use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::v1;
use crate::{data_display, data_try_from, default_version, identification::Identification};

pub const VERSION: u32 = 2;

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
    pub identifications: Vec<Identification>,
}

impl From<v1::Identity> for Identity {
    fn from(old: v1::Identity) -> Self {
        let identifications = vec![
            Identification {
                name: "registration".into(),
                value: old.identification.registration,
            },
            Identification {
                name: "tax".into(),
                value: old.identification.tax,
            },
        ];

        Self {
            _version: VERSION,
            id: old.id,
            name: old.name,
            address: old.address,
            phone: old.phone,
            email: old.email,
            www: old.www,
            identifications,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identities {
    pub identities: Vec<Identity>,
}

data_display!(Identity);
data_try_from!(Identity);
data_display!(Identities);
