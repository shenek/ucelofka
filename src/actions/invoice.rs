use std::path::Path;

use crate::data;

pub fn create<'a>(
    data_path: &'a Path,
    customer: &str,
    identity: &str,
    account: &str,
    entries: &[&str],
) {
}

pub fn list(data_path: &Path) {
    let invoice_path = data_path.join(Path::new("invoices"));
    println!("{}", data::invoice::Invoices::load(invoice_path.as_path()));
}
