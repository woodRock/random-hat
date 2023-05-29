/// Main - `main.rs`
/// ==============
/// The entry point for the applicaiton, it runs the application and sets up the router.
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod routes;
mod models;

use crate::routes::{switch, Route};

/// The App component is the root component of the application.
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

// The entry point of the web application.
fn main() {
    // Initialize the logger when the application is started.
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}