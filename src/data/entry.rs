use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};

use super::{list_directory, load_records};

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub price: f32,
    pub currency: String,
    pub details: Vec<String>,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Entries {
    pub entries: Vec<Entry>,
}

impl Entries {
    pub fn load(entries_dir: &Path) -> Self {
        let paths = list_directory(entries_dir);
        Self {
            entries: load_records::<Entry>(paths),
        }
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
