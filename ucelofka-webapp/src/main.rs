#![recursion_limit = "4096"]
mod components;

use console_error_panic_hook::hook;
use std::panic;

fn main() {
    panic::set_hook(Box::new(hook));

    yew::start_app::<components::App>();
}
