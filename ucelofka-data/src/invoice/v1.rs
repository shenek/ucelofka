#![allow(non_snake_case)]

use chrono::{Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{account, customer, data_display, data_try_from, default_version, entry, identity};

pub const VERSION: u32 = 1;

pub(crate) const DEFAULT_DUE: i64 = 15; // in days

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Billing {
    pub account_name: String,
    pub account_number: String,
    pub BIC: String,
    pub IBAN: String,
    pub total: f32,
    pub currency: String,
    pub variable_symbol: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Entry {
    pub name: String,
    pub price: f32,
    pub currency: String,
    pub details: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Customer {
    pub name: String,
    pub address: Vec<String>,
    pub identification: String,
    pub email: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identification {
    pub tax: String,
    pub registration: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Issuer {
    pub name: String,
    pub address: Vec<String>,
    pub phone: Vec<String>,
    pub email: Vec<String>,
    pub www: Vec<String>,
    pub identification: Identification,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Invoice {
    #[serde(default = "default_version")]
    pub _version: u32,
    pub id: u64,
    pub issue_day: String,
    pub due_day: String,
    pub issuer: Issuer,
    pub customer: Customer,
    pub entries: Vec<Entry>,
    pub billing: Billing,
}

impl Invoice {
    fn make_new_id(invoices: &[Self]) -> u64 {
        invoices
            .iter()
            .map(|i| i.id + 1)
            .max()
            .unwrap_or_else(|| Utc::now().date_naive().year() as u64 * 100_000 + 1)
    }

    pub fn new(
        identity: identity::v1::Identity,
        account: account::Account,
        customer: customer::v1::Customer,
        entries: &[entry::Entry],
        invoices: Vec<Self>,
    ) -> Self {
        let total = entries.iter().map(|e| e.price).sum();
        let new_id = Self::make_new_id(&invoices);
        Self {
            _version: VERSION,
            id: new_id,
            issue_day: Utc::now().format("%Y-%m-%d").to_string(),
            due_day: (Utc::now() + Duration::try_days(DEFAULT_DUE).unwrap_or_default())
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Invoices {
    pub invoices: Vec<Invoice>,
}

data_display!(Invoice);
data_try_from!(Invoice);
data_display!(Invoices);
