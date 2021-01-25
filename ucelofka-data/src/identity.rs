use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identification {
    pub tax: String,
    pub registration: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identity {
    pub id: String,
    pub name: String,
    pub address: Vec<String>,
    pub phone: Vec<String>,
    pub email: Vec<String>,
    pub www: Vec<String>,
    pub identification: Identification,
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identities {
    pub identities: Vec<Identity>,
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
