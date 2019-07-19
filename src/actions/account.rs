use std::path::Path;

use crate::data::account::Accounts;

pub fn list(data_path: &Path) {
    let account_path = data_path.join(Path::new("accounts"));
    println!("{}", Accounts::load(account_path.as_path()));
}
