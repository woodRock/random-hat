use crate::routes::Route;
/// Navigation - navigation.rs
/// ==========================
/// This is the navigation component. It is used to navigate between pages.
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    // Navigate to each route when the corresponding button is clicked.
    html! {
        <div>
            <ul>
                <li>
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                </li>
            </ul>
        </div>
    }
}