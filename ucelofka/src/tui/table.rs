#![allow(non_snake_case, deprecated)]

use dioxus::prelude::*;

#[inline_props]
pub fn Table<'a, T1, T2>(cx: Scope, width: &'a str, items: Vec<(T1, T2)>) -> Element
where
    T1: ToString,
    T2: ToString,
{
    cx.render(rsx! {
        ul { width: "{width}", justify_content: "center", flex_direction: "column",
            items.iter().map(|(k, v)| rsx!{
                li {
                    justify_content: "center",
                    div {
                        width: "50%",
                        margin_right: "1px",
                        div { width: "100%" }
                        strong { k.to_string() }
                    }
                    div {
                        width: "50%",
                        v.to_string()
                    }
                }

            })
        }
    })
}
