#![allow(non_snake_case, deprecated)]

use dioxus::{
    events::{KeyCode, KeyboardEvent},
    prelude::*,
};
use dioxus_tui::TuiContext;
use std::{path::PathBuf, rc::Rc};

use super::{
    account::Accounts, customer::Customers, entry::Entries, identity::Identities,
    invoice::Invoices, list::List,
};

struct CurrentPage(Page);

pub struct UcelofkaTuiCfg {
    pub path: PathBuf,
}

#[derive(Clone, Copy)]
enum Page {
    Accounts,
    Customers,
    Entries,
    Identities,
    Invoices,
}
impl AsRef<str> for Page {
    fn as_ref(&self) -> &str {
        match self {
            Self::Accounts => "Accounts",
            Self::Customers => "Customers",
            Self::Entries => "Entries",
            Self::Identities => "Identities",
            Self::Invoices => "Invoices",
        }
    }
}

impl From<usize> for Page {
    fn from(page: usize) -> Self {
        match page {
            0 => Self::Accounts,
            1 => Self::Customers,
            2 => Self::Entries,
            3 => Self::Identities,
            4 => Self::Invoices,
            _ => unreachable!(),
        }
    }
}

impl From<Page> for usize {
    fn from(val: Page) -> Self {
        match val {
            Page::Accounts => 0,
            Page::Customers => 1,
            Page::Entries => 2,
            Page::Identities => 3,
            Page::Invoices => 4,
        }
    }
}

#[inline_props]
pub fn App(cx: Scope, path: PathBuf) -> Element {
    let tui_ctx: TuiContext = cx.consume_context().unwrap();

    use_shared_state_provider(cx, || CurrentPage(Page::Accounts));
    use_shared_state_provider(cx, || UcelofkaTuiCfg { path: path.clone() });

    let page = use_shared_state::<CurrentPage>(cx).unwrap();

    let items = Rc::new(vec![
        Page::Accounts.as_ref(),
        Page::Customers.as_ref(),
        Page::Entries.as_ref(),
        Page::Identities.as_ref(),
        Page::Invoices.as_ref(),
    ]);

    let header_element = use_state(cx, || None::<Rc<MountedData>>);
    let onkeydown = move |k: KeyboardEvent| match k.key_code {
        KeyCode::Q => {
            tui_ctx.quit();
        }
        KeyCode::P => {
            if let Some(he) = header_element.as_ref() {
                let inner = he.clone();
                cx.spawn(async move {
                    let _ = inner.set_focus(true).await;
                });
            }
        }
        _ => {}
    };

    let onmounted = move |ctx: MountedEvent| {
        header_element.set(Some(ctx.inner().clone()));
        let inner = ctx.inner().clone();
        cx.spawn(async move {
            let _ = inner.set_focus(true).await;
        });
    };

    cx.render(rsx! {
        div {
            width: "100%",
            height: "100%",
            justify_content: "center",
            align_items: "center",
            onkeydown: onkeydown,
            onmounted: onmounted,
            tabindex: -1,
            List {
                tabindex: 0,
                width: "20%",
                items: items,
                dot: "❱",
                idx: page.read().0.into(),
                onindexupdate: move |i: usize| {
                    page.write().0 = i.into();
                }
            }
            match page.read().0 {
                Page::Accounts => {
                    rsx! {
                        Accounts {}
                    }
                }
                Page::Customers => {
                    rsx! {
                        Customers {}
                    }

                }
                Page::Entries => {
                    rsx! {
                        Entries {}
                    }
                }
                Page::Identities => {
                    rsx! {
                        Identities {}
                    }
                }
                Page::Invoices => {
                    rsx! {
                        Invoices {}
                    }
                }
            }
        }
    })
}
