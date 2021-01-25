use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub address: Vec<String>,
    pub identification: String,
    pub email: Vec<String>,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Customers {
    pub customers: Vec<Customer>,
}

impl fmt::Display for Customers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

impl TryFrom<String> for Customer {
    type Error = serde_yaml::Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Ok(serde_yaml::from_str(&input)?)
    }
}
