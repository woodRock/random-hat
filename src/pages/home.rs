// Home - `home.rs`
use crate::routes::Route;
use crate::models::student::{Student,read_students_from_json};

use yew::prelude::*;
use yew_router::prelude::*;
use yew::{html, Callback};

#[function_component(Home)]
pub fn home() -> Html {

    const DEFAULT: &str = "???";
    let selected_student = use_state(|| String::from(DEFAULT));

    let mut students: Vec<Student> = read_students_from_json();
    // let mut students: Vec<Student> = serde_json::from_str(STUDENTS_JSON).expect("failed to parse JSON");

    let index = (rand::random::<f32>() * students.len() as f32).floor() as usize;
    let selected = students.remove( index );
    let super_random_student: String = format!("{}: {}", selected.name, selected.topic);

    let select_random_student = {
        let selected_student = selected_student.clone();
        Callback::once(move |_| selected_student.set(super_random_student))
    };

    let reset = {
        let selected_student = selected_student.clone();
        Callback::from(move |_| selected_student.set(String::from(DEFAULT)))
    };

    let history = use_history().unwrap();
    let see_students = Callback::once(move |_| history.push(Route::Students));

    html! {
        <div>
            <header>
                <h1>{ "Random Hat 🎩" } </h1>
            </header>
            <hr/>
            <div id="description">
                <p> { "A random name generator for selecting FASLIP nominees each week."} </p>
                <h3> { "Click the big hat icon to (psuedo) randomly select a speaker for next week!" } </h3>
            </div>
            <div id="actions">
                <button onclick={select_random_student}> <h1> { "🎩" } </h1> </button>
                <p> </p>
                <button onclick={reset}> { "🔄" } </button>
            </div>
            <div id="selected-student">
                <h1> {selected_student.to_string()}</h1>
            </div>
            <hr/>
            <footer>
                <button onclick={see_students}>{"See FASLIP students"}</button>
            </footer>
        </div>
    }
}
