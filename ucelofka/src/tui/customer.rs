#![allow(non_snake_case, deprecated)]

use dioxus::prelude::*;
use std::rc::Rc;
use ucelofka_data::customer::Customer;

use crate::actions::customer::list;

use super::{app::UcelofkaTuiCfg, list::List, table::Table};

struct CurrentCustomerPage(usize);

#[derive(Clone)]
enum SubPage {
    Create,
    Customer(Customer),
}

impl AsRef<str> for SubPage {
    fn as_ref(&self) -> &str {
        match self {
            Self::Create => "<Create>",
            Self::Customer(customer) => customer.name.as_str(),
        }
    }
}

#[inline_props]
pub fn Customers(cx: Scope) -> Element {
    use_shared_state_provider(cx, || CurrentCustomerPage(0));

    let customer_page = use_shared_state::<CurrentCustomerPage>(cx).unwrap();

    let cfg = use_shared_state::<UcelofkaTuiCfg>(cx).unwrap();
    let customers = list(cfg.read().path.as_path()).unwrap().customers;
    let items: Vec<SubPage> = vec![SubPage::Create]
        .into_iter()
        .chain(customers.iter().map(|e| SubPage::Customer(e.clone())))
        .collect();

    let items_str: Vec<String> = items.iter().map(|e| e.as_ref().to_string()).collect();
    let selected_idx = customer_page.read().0;
    let selected_customer = if selected_idx == 0 {
        None
    } else {
        Some(customers[selected_idx - 1].clone())
    };

    cx.render(rsx! {
        List {
            tabindex: 0,
            width: "20%",
            items: Rc::new(items_str),
            dot: "❱",
            idx: customer_page.read().0,
            onindexupdate: move |i: usize| {
                customer_page.write().0 = i;
            }
        }
        div { width: "60%", border_width: "1px", height: "100%", justify_content: "center",
            if let Some(customer) = selected_customer {
                rsx! {
                    Table {
                        width: "100%",
                        items: vec![
                            ("Name:", customer.name),
                            ("Address:", customer.address.join(", ")),
                            ("Email:", customer.email.join(", ")),
                            ("IDs:", customer.identifications.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")),
                        ]
                    }
            }
            } else {
                rsx! {span { "TODO CREATE FORM"}}
            }
        }
    })
}
