#![allow(non_snake_case)]

use chrono::{Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, path::Path};
use tera::Context;

use super::{Record, Records};
use crate::data;

const DEFAULT_DUE: i64 = 15; // in days

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Billing {
    pub account_name: String,
    pub account_number: String,
    pub BIC: String,
    pub IBAN: String,
    pub total: f32,
    pub currency: String,
    pub variable_symbol: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    pub name: String,
    pub price: f32,
    pub currency: String,
    pub details: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Customer {
    pub name: String,
    pub address: Vec<String>,
    pub identification: String,
    pub email: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Identification {
    pub tax: String,
    pub registration: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Issuer {
    pub name: String,
    pub address: Vec<String>,
    pub phone: Vec<String>,
    pub email: Vec<String>,
    pub www: Vec<String>,
    pub identification: Identification,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Invoice {
    pub id: u64,
    pub issue_day: String,
    pub due_day: String,
    pub issuer: Issuer,
    pub customer: Customer,
    pub entries: Vec<Entry>,
    pub billing: Billing,
}

impl Record for Invoice {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
        Ok(())
    }
}

impl From<Invoice> for Context {
    fn from(invoice: Invoice) -> Self {
        Self::from_serialize(invoice).unwrap()
    }
}

impl Invoice {
    fn make_new_id(invoices: &[Self]) -> u64 {
        invoices
            .iter()
            .map(|i| i.id + 1)
            .max()
            .unwrap_or_else(|| Utc::today().naive_utc().year() as u64 * 100_000 + 1)
    }

    pub fn new(
        identity: data::identity::Identity,
        account: data::account::Account,
        customer: data::customer::Customer,
        entries: &[data::entry::Entry],
        invoices: Vec<Self>,
    ) -> Self {
        let total = entries.iter().map(|e| e.price).sum();
        let new_id = Self::make_new_id(&invoices);
        Self {
            id: new_id,
            issue_day: Utc::today().format("%Y-%m-%d").to_string(),
            due_day: (Utc::now() + Duration::days(DEFAULT_DUE))
                .format("%Y-%m-%d")
                .to_string(),
            issuer: Issuer {
                name: identity.name,
                address: identity.address,
                email: identity.email,
                phone: identity.phone,
                www: identity.www,
                identification: Identification {
                    tax: identity.identification.tax,
                    registration: identity.identification.registration,
                },
            },
            customer: Customer {
                name: customer.name,
                address: customer.address,
                identification: customer.identification,
                email: customer.email,
            },
            billing: Billing {
                account_name: account.account_name,
                account_number: account.account_number,
                BIC: account.BIC,
                IBAN: account.IBAN,
                total,
                currency: account.currency,
                variable_symbol: new_id.to_string(),
            },
            entries: entries
                .iter()
                .map(|e| Entry {
                    currency: e.currency.clone(),
                    price: e.price,
                    name: e.name.clone(),
                    details: e.details.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Invoices {
    pub invoices: Vec<Invoice>,
}

impl<'a> Records<'a, Invoice> for Invoices {
    fn new(invoices: Vec<Invoice>) -> Self {
        Self { invoices }
    }

    fn load(dir: &Path) -> Self {
        let paths = Self::list_directory(dir);
        Self::new(Self::load_records(paths))
    }

    fn records(&'a self) -> &'a [Invoice] {
        &self.invoices
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
