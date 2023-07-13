#![allow(dead_code)]

use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::{
    pages::NotFound,
    pages::Identities,
    components::Nav,
};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/identities")]
    Identities,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <main>
                <ybc::Columns>
                    <ybc::Column classes="is-3">
                        <Nav />
                    </ybc::Column>
                    <ybc::Column>
                        <ybc::Container>
                            <Switch<Route> render={switch} />
                        </ybc::Container>
                    </ybc::Column>
                </ybc::Columns>
            </main>
        </BrowserRouter>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <div /> }
        }
        Route::NotFound => {
            html! { <NotFound/> }
        }
        Route::Identities => {
            html! { <Identities/> }
        }
    }
}
