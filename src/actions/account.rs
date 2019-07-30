use std::path::Path;

use crate::data::{
    account::{Account, Accounts},
    Records,
};

pub fn list(data_path: &Path) -> Accounts {
    let account_path = data_path.join(Path::new("accounts"));
    Accounts::load(account_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Option<Account> {
    list(data_path).get(id)
}
