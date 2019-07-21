use std::path::Path;

use crate::data::account::{Account, Accounts};

pub fn list(data_path: &Path) -> Accounts {
    let account_path = data_path.join(Path::new("accounts"));
    Accounts::load(account_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Option<Account> {
    for account in list(data_path).accounts {
        if account.id == id {
            return Some(account);
        }
    }
    None
}
