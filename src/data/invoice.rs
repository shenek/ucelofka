#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};
use tera::Context;

use super::{list_directory, load_records};

#[derive(Debug, Deserialize, Serialize)]
struct Billing {
    account_name: String,
    account_number: String,
    BIC: String,
    IBAN: String,
    total: f32,
    currency: String,
    variable_symbol: String,
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
    email: Vec<String>,
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
    issue_day: String,
    due_day: String,
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

impl From<&Invoice> for Context {
    fn from(invoice: &Invoice) -> Self {
        let mut context: Self = Self::from_serialize(invoice).unwrap();
        context.insert("aaa", "bbb");
        context
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Invoices {
    invoices: Vec<Invoice>,
}

impl Invoices {
    pub fn load(invoice_dir: &Path) -> Self {
        let paths = list_directory(invoice_dir);
        Self {
            invoices: load_records::<Invoice>(paths),
        }
    }

    pub fn get<'a>(&'a self, id: u64) -> Option<&'a Invoice> {
        for invoice in &self.invoices {
            if invoice.id == id {
                return Some(invoice);
            }
        }
        None
    }
}

impl fmt::Display for Invoices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

impl TryFrom<String> for Invoice {
    type Error = serde_yaml::Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Ok(serde_yaml::from_str(&input)?)
    }
}
