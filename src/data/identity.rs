use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};

use super::{Record, Records};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Identification {
    pub tax: String,
    pub registration: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Identity {
    pub id: String,
    pub name: String,
    pub address: Vec<String>,
    pub phone: Vec<String>,
    pub email: Vec<String>,
    pub www: Vec<String>,
    pub identification: Identification,
}

impl Record for Identity {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Identities {
    pub identities: Vec<Identity>,
}

impl<'a> Records<'a, Identity> for Identities {
    fn new(identities: Vec<Identity>) -> Self {
        Self { identities }
    }

    fn load(dir: &Path) -> Self {
        let paths = Self::list_directory(dir);
        Self::new(Self::load_records(paths))
    }

    fn records(&'a self) -> &'a [Identity] {
        &self.identities
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
