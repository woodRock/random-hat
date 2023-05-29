use crate::components::return_home::ReturnHome;
/// Error - error.rs
/// ===============
/// This is the error page. It is displayed when the user tries to access a page that does not exist.
use yew::prelude::*;

#[function_component(Error)]
pub fn error() -> Html {
    html! {
        <div>
            <h1>{ "404" }</h1>
            <p>{ "Page not found." }</p>
            <ReturnHome />
        </div>
    }
}
