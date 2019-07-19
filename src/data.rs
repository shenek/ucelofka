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

use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

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

fn load_records<T>(paths: Vec<PathBuf>) -> Vec<T>
where
    T: TryFrom<String>,
{
    let mut res: Vec<T> = Vec::new();

    for path in paths {
        let parsed: T = match std::fs::read_to_string(path) {
            Ok(content) => {
                let item_opt: Result<T, _> = T::try_from(content);
                match item_opt {
                    Ok(item) => item,
                    Err(err) => panic!(format!("failed to convert")),
                }
            }
            Err(err) => panic!(format!("{}", err)),
        };
        res.push(parsed);
    }
    res
}
