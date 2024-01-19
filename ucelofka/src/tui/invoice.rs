#![allow(non_snake_case, deprecated, clippy::large_enum_variant)]

use dioxus::prelude::*;
use std::rc::Rc;
use ucelofka_data::invoice::Invoice;

use crate::actions::invoice::list;

use super::{app::UcelofkaTuiCfg, list::List, table::Table};

struct CurrentInvoicePage(usize);

#[derive(Clone)]
enum SubPage {
    Create,
    Invoice(Invoice),
}

impl ToString for SubPage {
    fn to_string(&self) -> String {
        match self {
            Self::Create => "<Create>".to_owned(),
            Self::Invoice(invoice) => invoice.id.to_string(),
        }
    }
}

#[inline_props]
pub fn Invoices(cx: Scope) -> Element {
    use_shared_state_provider(cx, || CurrentInvoicePage(0));

    let invoice_page = use_shared_state::<CurrentInvoicePage>(cx).unwrap();

    let cfg = use_shared_state::<UcelofkaTuiCfg>(cx).unwrap();
    let invoices = list(cfg.read().path.as_path()).unwrap().invoices;
    let items: Vec<SubPage> = vec![SubPage::Create]
        .into_iter()
        .chain(invoices.iter().map(|e| SubPage::Invoice(e.clone())))
        .collect();

    let items_str: Vec<String> = items.iter().map(|e| e.to_string()).collect();
    let selected_idx = invoice_page.read().0;
    let selected_invoice = if selected_idx == 0 {
        None
    } else {
        Some(invoices[selected_idx - 1].clone())
    };

    let get_items = |invoice: Invoice| {
        vec![
            ("ID:", invoice.id.to_string()),
            ("Issue date:", invoice.issue_date),
            ("Due date:", invoice.due_date),
        ]
    };

    cx.render(rsx! {
        List {
            tabindex: 0,
            width: "20%",
            items: Rc::new(items_str),
            dot: "❱",
            idx: invoice_page.read().0,
            onindexupdate: move |i: usize| {
                invoice_page.write().0 = i;
            }
        }
        div { width: "60%", border_width: "1px", height: "100%", justify_content: "center",
            if let Some(invoice) = selected_invoice {
                rsx! {
                    Table {
                        width: "100%",
                        items: get_items(invoice),
                    }
            }
            } else {
                rsx! {span { "TODO CREATE FORM"}}
            }
        }
    })
}
