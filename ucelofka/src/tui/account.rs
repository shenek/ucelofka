#![allow(non_snake_case, deprecated)]

use dioxus::prelude::*;
use std::rc::Rc;
use ucelofka_data::account::Account;

use crate::actions::account::list;

use super::{app::UcelofkaTuiCfg, list::List, table::Table};

struct CurrentAccountPage(usize);

#[derive(Clone)]
enum SubPage {
    Create,
    Account(Account),
}

impl AsRef<str> for SubPage {
    fn as_ref(&self) -> &str {
        match self {
            Self::Create => "<Create>",
            Self::Account(account) => account.name.as_str(),
        }
    }
}

#[inline_props]
pub fn Accounts(cx: Scope) -> Element {
    use_shared_state_provider(cx, || CurrentAccountPage(0));

    let account_page = use_shared_state::<CurrentAccountPage>(cx).unwrap();

    let cfg = use_shared_state::<UcelofkaTuiCfg>(cx).unwrap();
    let accounts = list(cfg.read().path.as_path()).unwrap().accounts;
    let items: Vec<SubPage> = vec![SubPage::Create]
        .into_iter()
        .chain(accounts.iter().map(|e| SubPage::Account(e.clone())))
        .collect();

    let items_str: Vec<String> = items.iter().map(|e| e.as_ref().to_string()).collect();
    let selected_idx = account_page.read().0;
    let selected_account = if selected_idx == 0 {
        None
    } else {
        Some(accounts[selected_idx - 1].clone())
    };

    cx.render(rsx! {
        List {
            tabindex: 0,
            width: "20%",
            items: Rc::new(items_str),
            dot: "❱",
            idx: account_page.read().0.into(),
            onindexupdate: move |i: usize| {
                account_page.write().0 = i.into();
            }
        }
        div { width: "60%", border_width: "1px", height: "100%", justify_content: "center",
            if let Some(account) = selected_account {
                rsx! {
                    Table {
                        width: "100%",
                        items: vec![
                            ("Name:", account.name),
                            ("Bank Name:", account.bank_name),
                            ("Account Name:", account.account_name),
                            ("Account number:", account.account_number),
                            ("IBAN:", account.IBAN),
                            ("BIC:", account.BIC),
                            ("Currency:", account.currency),

                        ]
                    }
            }
            } else {
                rsx! {span { "TODO CREATE FORM"}}
            }
        }
    })
}
