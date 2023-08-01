#![allow(non_snake_case, deprecated)]

use dioxus::{
    events::{KeyCode, KeyboardEvent},
    prelude::*,
};

use std::rc::Rc;

#[inline_props]
pub fn List<'a, T>(
    cx: Scope,
    width: Option<&'a str>,
    idx: usize,
    tabindex: i64,
    dot: Option<&'a str>,
    onindexupdate: EventHandler<'a, usize>,
    items: Rc<Vec<T>>,
) -> Element
where
    T: ToString,
{
    let width = width.unwrap_or_else(|| "100%");
    let item_len = items.len();
    let dot = dot.unwrap_or_default();

    let onkeydown = move |k: KeyboardEvent| {
        match k.key_code {
            KeyCode::UpArrow => {
                let new_index = if *idx == 0 { item_len - 1 } else { idx - 1 };
                onindexupdate.call(new_index);
            }
            KeyCode::DownArrow => {
                let new_index = if *idx == item_len - 1 { 0 } else { idx + 1 };
                onindexupdate.call(new_index);
            }
            _ => {}
        };
        k.stop_propagation();
    };

    cx.render(rsx! {
        ListInner {
            tabindex: *tabindex,
            width: width,
            idx: *idx,
            dot: dot,
            items: items.clone(),
            onkeydown: onkeydown
        }
    })
}

#[inline_props]
pub fn ListInner<'a, T>(
    cx: Scope,
    idx: usize,
    dot: &'a str,
    width: &'a str,
    tabindex: i64,
    items: Rc<Vec<T>>,
    onkeydown: EventHandler<'a, KeyboardEvent>,
) -> Element
where
    T: ToString,
{
    let items: Vec<_> = items.iter().enumerate().collect();

    cx.render(rsx! {
        ul {
            width: *width,
            tabindex: *tabindex,
            onkeydown: move |k: KeyboardEvent| onkeydown.call(k),
            flex_direction: "column",
            items.iter().map(move |(i, text)| {
                let background_color = if *idx == *i {
                    "blue"
                } else { ""};
                rsx! {
                    li {
                        background_color: "{background_color}",
                        width: "100%",
                        if *idx == *i {
                            format!("{}{}", dot, text.to_string())
                        } else {
                            text.to_string()
                        }
                    }
                }
            })
        }
    })
}
