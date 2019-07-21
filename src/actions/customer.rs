use std::path::Path;

use crate::data::customer::{Customer, Customers};

pub fn list(data_path: &Path) -> Customers {
    let customer_path = data_path.join(Path::new("customers"));
    Customers::load(customer_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Option<Customer> {
    for customer in list(data_path).customers {
        if customer.id == id {
            return Some(customer);
        }
    }
    None
}
