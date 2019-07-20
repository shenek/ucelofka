use std::path::Path;

use crate::data::customer::Customers;

pub fn list(data_path: &Path) {
    let customer_path = data_path.join(Path::new("customers"));
    println!("{}", Customers::load(customer_path.as_path()));
}
