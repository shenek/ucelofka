use std::path::Path;

#[derive(Debug)]
pub struct Customer {}

#[derive(Debug, Default)]
pub struct Customers {
    customers: Vec<Customer>,
}

impl Customers {
    pub fn load(path: &Path) -> Self {
        Self::default()
    }
}
