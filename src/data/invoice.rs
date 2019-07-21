#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};
use tera::Context;

use super::{list_directory, load_records};

#[derive(Debug, Deserialize, Serialize)]
pub struct Billing {
    pub account_name: String,
    pub account_number: String,
    pub BIC: String,
    pub IBAN: String,
    pub total: f32,
    pub currency: String,
    pub variable_symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub name: String,
    pub price: f32,
    pub currency: String,
    pub details: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    pub name: String,
    pub address: Vec<String>,
    pub identification: String,
    pub email: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identification {
    pub tax: String,
    pub registration: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Issuer {
    pub name: String,
    pub address: Vec<String>,
    pub phone: Vec<String>,
    pub email: Vec<String>,
    pub www: Vec<String>,
    pub identification: Identification,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Invoice {
    pub id: u64,
    pub issue_day: String,
    pub due_day: String,
    pub issuer: Issuer,
    pub customer: Customer,
    pub entries: Vec<Entry>,
    pub billing: Billing,
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
    pub invoices: Vec<Invoice>,
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
