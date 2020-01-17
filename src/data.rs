pub mod account;
pub mod customer;
pub mod entry;
pub mod identity;
pub mod invoice;
pub mod template;

pub use account::Accounts;
pub use customer::Customers;
pub use entry::Entries;
pub use identity::Identities;
pub use invoice::Invoices;
pub use template::Templates;

use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{
    convert::TryFrom,
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

pub trait Record: Serialize + Debug {
    fn id(&self) -> String
    where
        Self: Serialize;

    fn filename(&self) -> String {
        format!("{}.yml", self.id())
    }

    fn store(&self, dir: &Path) -> Result<()> {
        let serialzed = serde_yaml::to_string(self)?;
        fs::write(dir.join(Path::new(&self.filename())), serialzed)?;

        Ok(())
    }
}

pub trait Records<'a, ITEM>: Serialize + Debug
where
    ITEM: 'a + TryFrom<String> + Clone + Record,
    <ITEM as TryFrom<String>>::Error: std::fmt::Debug,
{
    fn new(records: Vec<ITEM>) -> Self;
    fn load(dir: &Path) -> Result<Self>
    where
        Self: Sized;
    fn records(&'a self) -> &'a [ITEM];

    fn get(&'a self, id: &str) -> Option<ITEM> {
        for record in self.records() {
            if record.id() == id {
                return Some(record.clone());
            }
        }
        None
    }

    fn list_directory(dir: &Path) -> Result<Vec<PathBuf>> {
        let mut res: Vec<PathBuf> = dir
            .read_dir()?
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|e| dir.join(e.path()))
            .collect();
        // sort by filename
        res.sort();
        Ok(res)
    }

    fn load_records(paths: Vec<PathBuf>) -> Result<Vec<ITEM>> {
        let mut res: Vec<ITEM> = Vec::new();

        for path in paths {
            let path_str = path
                .to_str()
                .ok_or_else(|| anyhow!("Invalid path {}", path.to_string_lossy()))?
                .to_string();
            let data = std::fs::read_to_string(path)?;
            let parsed = ITEM::try_from(data)
                .map_err(|err| anyhow!("failed to convert {} - {:?}", path_str, err))?;
            res.push(parsed);
        }
        Ok(res)
    }
}
