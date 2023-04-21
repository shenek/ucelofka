#![recursion_limit = "4096"]
mod components;
mod app;

use yew::prelude::*;

fn main() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::Renderer::<app::App>::new().render();
}
