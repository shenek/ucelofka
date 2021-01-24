pub use ucelofka_data::{
    account::{Account, Accounts},
    customer::{Customer, Customers},
    entry::{Entries, Entry},
    identity::{Identities, Identity},
    invoice::{Invoice, Invoices},
    template::{Template, Templates},
};

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
        fs::write(dir.join(Path::new(&self.filename())), serialzed + "\n")?;

        Ok(())
    }
}

pub trait Records<ITEM>: Serialize + Debug
where
    ITEM: TryFrom<String> + Clone + Record,
    <ITEM as TryFrom<String>>::Error: std::fmt::Debug,
{
    fn new(records: Vec<ITEM>) -> Self;
    fn load(dir: &Path) -> Result<Self>
    where
        Self: Sized;
    fn records(&self) -> &[ITEM];

    fn get(&self, id: &str) -> Option<ITEM> {
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

    fn ids(&self) -> Vec<String> {
        self.records().iter().map(|r| r.id()).collect()
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

impl Record for Account {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Records<Account> for Accounts {
    fn new(accounts: Vec<Account>) -> Self {
        Self { accounts }
    }

    fn load(dir: &Path) -> Result<Self> {
        let paths = Self::list_directory(dir)?;
        Ok(Self::new(Self::load_records(paths)?))
    }

    fn records(&self) -> &[Account] {
        &self.accounts
    }
}

impl Record for Customer {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Records<Customer> for Customers {
    fn new(customers: Vec<Customer>) -> Self {
        Self { customers }
    }

    fn load(dir: &Path) -> Result<Self> {
        let paths = Self::list_directory(dir)?;
        Ok(Self::new(Self::load_records(paths)?))
    }

    fn records(&self) -> &[Customer] {
        &self.customers
    }
}

impl Record for Entry {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Records<Entry> for Entries {
    fn new(entries: Vec<Entry>) -> Self {
        Self { entries }
    }

    fn load(dir: &Path) -> Result<Self> {
        let paths = Self::list_directory(dir)?;
        Ok(Self::new(Self::load_records(paths)?))
    }

    fn records(&self) -> &[Entry] {
        &self.entries
    }
}

impl Record for Identity {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Records<Identity> for Identities {
    fn new(identities: Vec<Identity>) -> Self {
        Self { identities }
    }

    fn load(dir: &Path) -> Result<Self> {
        let paths = Self::list_directory(dir)?;
        Ok(Self::new(Self::load_records(paths)?))
    }

    fn records(&self) -> &[Identity] {
        &self.identities
    }
}

impl Record for Invoice {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

impl Records<Invoice> for Invoices {
    fn new(invoices: Vec<Invoice>) -> Self {
        Self { invoices }
    }

    fn load(dir: &Path) -> Result<Self> {
        let paths = Self::list_directory(dir)?;
        Ok(Self::new(Self::load_records(paths)?))
    }

    fn records(&self) -> &[Invoice] {
        &self.invoices
    }
}
