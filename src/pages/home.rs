use crate::routes::Route;
/// Home - home.rs
/// ==============
/// This is the home page. It is the first page that the user sees when they visit the website.
use yew::prelude::*;
use yew_router::prelude::*;
// use log::info;

use yew::{html, Callback};

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let see_students = Callback::once(move |_| history.push(Route::Students));

    const DEFAULT: &str = "???";
    const SUPER_RANDOM_STUDENT: &str = "Jordan MacLachlan: Evolutionary Computation for Emergency Medical Services Routing";

    let selected_student = use_state(|| String::from(DEFAULT));
    let onclick = {
        let selected_student = selected_student.clone();
        Callback::from(move |_| selected_student.set(String::from(SUPER_RANDOM_STUDENT)))
    };
    let reset = {
        let selected_student = selected_student.clone();
        Callback::from(move |_| selected_student.set(String::from(DEFAULT)))
    };

    html! {
        <div>
            <h1>{ "Random Hat 🎩" } </h1>
            <p> { "A random name generator for selecting FASLIP nominees each week."} </p>
            <h3> { "Click the big hat icon to (psuedo) randomly select a speaker for next week!" } </h3>
            <button onclick={onclick}> <h1> { "🎩" } </h1> </button>
            <h1> {selected_student.to_string()}</h1>
            <button onclick={reset}>{"Reset"}</button>
            <p></p>
            <button onclick={see_students}>{"See FASLIP students"}</button>
        </div>
    }
}
