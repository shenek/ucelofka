use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};

use super::{list_directory, load_records};

#[derive(Debug, Deserialize, Serialize)]
pub struct Identification {
    tax: String,
    registration: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identity {
    name: String,
    address: Vec<String>,
    phone: Vec<String>,
    email: Vec<String>,
    www: Vec<String>,
    identification: Identification,
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Identities {
    identities: Vec<Identity>,
}

impl Identities {
    pub fn load(identity_dir: &Path) -> Self {
        let paths = list_directory(identity_dir);
        Self {
            identities: load_records::<Identity>(paths),
        }
    }
}

impl fmt::Display for Identities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

impl TryFrom<String> for Identity {
    type Error = serde_yaml::Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Ok(serde_yaml::from_str(&input)?)
    }
}
