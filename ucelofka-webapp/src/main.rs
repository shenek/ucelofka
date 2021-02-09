#![recursion_limit = "4096"]
mod components;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    yew::start_app::<components::App>();
}
