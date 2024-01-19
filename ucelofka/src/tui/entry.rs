#![allow(non_snake_case, deprecated)]

use dioxus::prelude::*;
use std::rc::Rc;
use ucelofka_data::entry::Entry;

use crate::actions::entry::list;

use super::{app::UcelofkaTuiCfg, list::List, table::Table};

struct CurrentEntryPage(usize);

#[derive(Clone)]
enum SubPage {
    Create,
    Entry(Entry),
}

impl AsRef<str> for SubPage {
    fn as_ref(&self) -> &str {
        match self {
            Self::Create => "<Create>",
            Self::Entry(entry) => entry.name.as_str(),
        }
    }
}

#[inline_props]
pub fn Entries(cx: Scope) -> Element {
    use_shared_state_provider(cx, || CurrentEntryPage(0));

    let entry_page = use_shared_state::<CurrentEntryPage>(cx).unwrap();

    let cfg = use_shared_state::<UcelofkaTuiCfg>(cx).unwrap();
    let entries = list(cfg.read().path.as_path()).unwrap().entries;
    let items: Vec<SubPage> = vec![SubPage::Create]
        .into_iter()
        .chain(entries.iter().map(|e| SubPage::Entry(e.clone())))
        .collect();

    let items_str: Vec<String> = items.iter().map(|e| e.as_ref().to_string()).collect();
    let selected_idx = entry_page.read().0;
    let selected_entry = if selected_idx == 0 {
        None
    } else {
        Some(entries[selected_idx - 1].clone())
    };

    let get_items = |entry: Entry| {
        vec![
            ("Name:", entry.name),
            ("Price:", entry.price.to_string()),
            ("Currency:", entry.currency),
        ]
        .into_iter()
        .chain(entry.details.into_iter().enumerate().map(|(idx, e)| {
            if idx == 0 {
                ("Details:", e)
            } else {
                ("", e)
            }
        }))
        .collect::<Vec<_>>()
    };

    cx.render(rsx! {
        List {
            tabindex: 0,
            width: "20%",
            items: Rc::new(items_str),
            dot: "❱",
            idx: entry_page.read().0,
            onindexupdate: move |i: usize| {
                entry_page.write().0 = i;
            }
        }
        div { width: "60%", border_width: "1px", height: "100%", justify_content: "center",
            if let Some(entry) = selected_entry {
                rsx! {
                    Table {
                        width: "100%",
                        items: get_items(entry),
                    }
            }
            } else {
                rsx! {span { "TODO CREATE FORM"}}
            }
        }
    })
}
