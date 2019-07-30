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

use serde::Serialize;
use std::{
    convert::TryFrom,
    fmt::Debug,
    fs, io,
    path::{Path, PathBuf},
};

pub trait Record: Serialize + Debug {
    fn id(&self) -> String
    where
        Self: Serialize;

    fn filename(&self) -> String {
        format!("{}.yml", self.id())
    }

    fn store(&self, dir: &Path) -> Result<(), io::Error> {
        let serialzed = serde_yaml::to_string(self).unwrap();
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
    fn load(dir: &Path) -> Self;
    fn records(&'a self) -> &'a [ITEM];

    fn get(&'a self, id: &str) -> Option<ITEM> {
        for record in self.records() {
            if record.id() == id {
                return Some(record.clone());
            }
        }
        None
    }

    fn list_directory(dir: &Path) -> Vec<PathBuf> {
        let mut res: Vec<PathBuf> = match dir.read_dir() {
            Ok(list) => {
                let res: Vec<PathBuf> = list
                    .map(|e| match e {
                        Ok(item) => dir.join(item.path()),
                        Err(err) => panic!(format!("{}", err)),
                    })
                    .collect();
                res
            }
            Err(err) => panic!(format!("{}", err)),
        };
        // sort by filename
        res.sort();
        res
    }

    fn load_records(paths: Vec<PathBuf>) -> Vec<ITEM> {
        let mut res: Vec<ITEM> = Vec::new();

        for path in paths {
            let path_str = path.to_str().unwrap().to_string();
            let parsed: ITEM = match std::fs::read_to_string(path) {
                Ok(content) => {
                    let item_opt: Result<ITEM, _> = ITEM::try_from(content);
                    match item_opt {
                        Ok(item) => item,
                        Err(err) => panic!(format!("failed to convert {} - {:?}", path_str, err)),
                    }
                }
                Err(err) => panic!(format!("{}", err)),
            };
            res.push(parsed);
        }
        res
    }
}
