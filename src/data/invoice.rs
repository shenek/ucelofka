#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize)]
struct Billing {
    account_name: String,
    account_number: String,
    BIC: String,
    IBAN: String,
    total: f32,
    currency: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    name: String,
    price: f32,
    currency: String,
    details: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Customer {
    name: String,
    address: Vec<String>,
    identification: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Identification {
    tax: String,
    registration: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Issuer {
    name: String,
    address: Vec<String>,
    phone: Vec<String>,
    email: Vec<String>,
    www: Vec<String>,
    identification: Identification,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Invoice {
    id: u64,
    issuer: Issuer,
    customer: Customer,
    entries: Vec<Entry>,
    billing: Billing,
}

impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Invoices {
    invoices: Vec<Invoice>,
}

impl Invoices {
    pub fn load(invoice_dir: &Path) -> Self {
        let mut paths: Vec<PathBuf> = match invoice_dir.read_dir() {
            Ok(list) => {
                let res: Vec<PathBuf> = list
                    .map(|e| match e {
                        Ok(item) => invoice_dir.join(item.path()),
                        Err(err) => panic!(format!("{}", err)),
                    })
                    .collect();
                res
            }
            Err(err) => panic!(format!("{}", err)),
        };
        // sort novices by filename
        paths.sort();

        let mut invoices: Vec<Invoice> = Vec::new();
        for path in paths {
            let invoice: Invoice = match std::fs::read_to_string(path) {
                Ok(content) => match serde_yaml::from_str(&content[..]) {
                    Ok(invoice) => invoice,
                    Err(err) => panic!(format!("{}", err)),
                },
                Err(err) => panic!(format!("{}", err)),
            };
            invoices.push(invoice);
        }
        Self { invoices }
    }
}

impl fmt::Display for Invoices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}
