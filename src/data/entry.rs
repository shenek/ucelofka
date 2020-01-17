use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};

use super::{Record, Records};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
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
            id,
            name,
            price,
            currency,
            details,
        }
    }
}

impl Record for Entry {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Entries {
    pub entries: Vec<Entry>,
}

impl<'a> Records<'a, Entry> for Entries {
    fn new(entries: Vec<Entry>) -> Self {
        Self { entries }
    }

    fn load(dir: &Path) -> Result<Self> {
        let paths = Self::list_directory(dir)?;
        Ok(Self::new(Self::load_records(paths)?))
    }

    fn records(&'a self) -> &'a [Entry] {
        &self.entries
    }
}

impl fmt::Display for Entries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

impl TryFrom<String> for Entry {
    type Error = serde_yaml::Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Ok(serde_yaml::from_str(&input)?)
    }
}
