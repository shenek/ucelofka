#![allow(non_snake_case)]

use chrono::{Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{account, customer, data_display, data_try_from, default_version, entry, identity};

use super::v1::{self, DEFAULT_DUE};
pub use super::v1::{Billing, Entry};
pub use crate::identification::v1::Identification;

pub const VERSION: u32 = 2;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Customer {
    pub name: String,
    pub address: Vec<String>,
    pub identifications: Vec<Identification>,
    pub email: Vec<String>,
}

impl From<v1::Customer> for Customer {
    fn from(old: v1::Customer) -> Self {
        let identifications = old
            .identification
            .splitn(2, '/')
            .map(|e| e.trim())
            .zip(["registration", "tax"].iter())
            .map(|(value, name)| Identification {
                name: name.to_string(),
                value: value.to_string(),
            })
            .collect();

        Self {
            name: old.name,
            address: old.address,
            email: old.email,
            identifications,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Issuer {
    pub name: String,
    pub address: Vec<String>,
    pub phone: Vec<String>,
    pub email: Vec<String>,
    pub www: Vec<String>,
    pub identifications: Vec<Identification>,
}

impl From<v1::Issuer> for Issuer {
    fn from(old: v1::Issuer) -> Self {
        let identifications = vec![
            Identification {
                name: "registration".into(),
                value: old.identification.registration,
            },
            Identification {
                name: "tax".into(),
                value: old.identification.tax,
            },
        ];
        Self {
            name: old.name,
            address: old.address,
            phone: old.phone,
            email: old.email,
            www: old.www,
            identifications,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Invoice {
    #[serde(default = "default_version")]
    pub _version: u32,
    pub id: u64,
    pub issue_date: String,
    pub due_date: String,
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
        identity: identity::Identity,
        account: account::Account,
        customer: customer::Customer,
        entries: &[entry::Entry],
        invoices: Vec<Self>,
        due: Option<usize>,
    ) -> Self {
        let total = entries.iter().map(|e| e.price).sum();
        let new_id = Self::make_new_id(&invoices);
        Self {
            _version: VERSION,
            id: new_id,
            issue_date: Utc::now().format("%Y-%m-%d").to_string(),
            due_date: (Utc::now()
                + Duration::days(
                    due.map(|v| i64::try_from(v).unwrap())
                        .unwrap_or(DEFAULT_DUE),
                ))
            .format("%Y-%m-%d")
            .to_string(),
            issuer: Issuer {
                name: identity.name,
                address: identity.address,
                email: identity.email,
                phone: identity.phone,
                www: identity.www,
                identifications: identity.identifications,
            },
            customer: Customer {
                name: customer.name,
                address: customer.address,
                identifications: customer.identifications,
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

impl From<v1::Invoice> for Invoice {
    fn from(old: v1::Invoice) -> Self {
        Self {
            _version: VERSION,
            id: old.id,
            issue_date: old.issue_day,
            due_date: old.due_day,
            entries: old.entries,
            billing: old.billing,
            issuer: old.issuer.into(),
            customer: old.customer.into(),
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
