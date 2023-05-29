use crate::pages;
/// Routes - `mod.rs`
/// ===============
/// We seperate the routing logic from the rest of the application.
/// This file is like __init__.py in Python. It is the entry point for the module.
/// It makes the each route module available to the rest of the application.
use yew::prelude::*;
use yew_router::prelude::*;

/// The route enum is used to define the routes for the application.
#[derive(Clone, Routable, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/Students")]
    Students,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// The switch component controls the routing of the application.
#[allow(clippy::let_unit_value)] // See https://github.com/rust-lang/rust-clippy/issues/9048
pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<pages::home::Home />},
        Route::Students => html! { <pages::students::Students />},
        Route::NotFound => html! { <pages::error::Error />},
    }
}