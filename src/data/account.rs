use std::path::Path;

#[derive(Debug)]
pub struct Account {}

#[derive(Debug, Default)]
pub struct Accounts {
    accounts: Vec<Account>,
}

impl Accounts {
    pub fn load(path: &Path) -> Self {
        Self::default()
    }
}
