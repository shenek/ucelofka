pub mod account;
pub mod customer;
pub mod entry;
pub mod identity;
pub mod invoice;

use std::path::Path;

use account::Accounts;
use customer::Customers;
use entry::Entries;
use identity::Identities;
use invoice::Invoices;

#[derive(Default, Debug)]
pub struct Data {
    accounts: Accounts,
    customers: Customers,
    entries: Entries,
    identities: Identities,
    invoices: Invoices,
}

impl Data {
    pub fn new(path: &Path) -> Self {
        Self {
            accounts: Accounts::load(path),
            customers: Customers::load(path),
            entries: Entries::load(path),
            identities: Identities::load(path),
            invoices: Invoices::load(path),
        }
    }
}
