#![allow(non_snake_case, deprecated)]

use dioxus::prelude::*;
use std::rc::Rc;
use ucelofka_data::identity::Identity;

use crate::actions::identity::list;

use super::{app::UcelofkaTuiCfg, list::List, table::Table};

struct CurrentIdentityPage(usize);

#[derive(Clone)]
enum SubPage {
    Create,
    Identity(Identity),
}

impl AsRef<str> for SubPage {
    fn as_ref(&self) -> &str {
        match self {
            Self::Create => "<Create>",
            Self::Identity(identity) => identity.name.as_str(),
        }
    }
}

#[inline_props]
pub fn Identities(cx: Scope) -> Element {
    use_shared_state_provider(cx, || CurrentIdentityPage(0));

    let identity_page = use_shared_state::<CurrentIdentityPage>(cx).unwrap();

    let cfg = use_shared_state::<UcelofkaTuiCfg>(cx).unwrap();
    let identities = list(cfg.read().path.as_path()).unwrap().identities;
    let items: Vec<SubPage> = vec![SubPage::Create]
        .into_iter()
        .chain(identities.iter().map(|e| SubPage::Identity(e.clone())))
        .collect();

    let items_str: Vec<String> = items.iter().map(|e| e.as_ref().to_string()).collect();
    let selected_idx = identity_page.read().0;
    let selected_identity = if selected_idx == 0 {
        None
    } else {
        Some(identities[selected_idx - 1].clone())
    };

    let get_items = |identity: Identity| {
        vec![("Name:", identity.name)]
            .into_iter()
            .chain(identity.address.into_iter().enumerate().map(|(idx, e)| {
                if idx == 0 {
                    ("Address:", e)
                } else {
                    ("", e)
                }
            }))
            .chain(identity.phone.into_iter().enumerate().map(|(idx, e)| {
                if idx == 0 {
                    ("Phone:", e)
                } else {
                    ("", e)
                }
            }))
            .chain(identity.email.into_iter().enumerate().map(|(idx, e)| {
                if idx == 0 {
                    ("Email:", e)
                } else {
                    ("", e)
                }
            }))
            .chain(identity.www.into_iter().enumerate().map(|(idx, e)| {
                if idx == 0 {
                    ("WWW:", e)
                } else {
                    ("", e)
                }
            }))
            .chain(
                identity
                    .identifications
                    .into_iter()
                    .enumerate()
                    .map(|(idx, e)| {
                        if idx == 0 {
                            ("Identification:", format!("{} - {}", e.name, e.value))
                        } else {
                            ("", format!("{} - {}", e.name, e.value))
                        }
                    }),
            )
            .collect::<Vec<_>>()
    };

    cx.render(rsx! {
        List {
            tabindex: 0,
            width: "20%",
            items: Rc::new(items_str),
            dot: "❱",
            idx: identity_page.read().0,
            onindexupdate: move |i: usize| {
                identity_page.write().0 = i;
            }
        }
        div { width: "60%", border_width: "1px", height: "100%", justify_content: "center",
            if let Some(identity) = selected_identity {
                rsx! {
                    Table {
                        width: "100%",
                        items: get_items(identity),
                    }
            }
            } else {
                rsx! {span { "TODO CREATE FORM"}}
            }
        }
    })
}
