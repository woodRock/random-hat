use crate::routes::Route;
use crate::models::student::{Student,STUDENTS_JSON};

use yew::prelude::*;
use yew_router::prelude::*;
use yew::{html, Callback};

#[function_component(Home)]
pub fn home() -> Html {

    const DEFAULT: &str = "???";
    let selected_student = use_state(|| String::from(DEFAULT));

    let mut students: Vec<Student> = serde_json::from_str(STUDENTS_JSON).expect("failed to parse JSON");

    let index = (rand::random::<f32>() * students.len() as f32).floor() as usize;
    let selected = students.remove( index );
    let super_random_student: String = format!("{}: {}", selected.name, selected.topic).to_string();

    let select_random_student = {
        let selected_student = selected_student.clone();
        Callback::once(move |_| selected_student.set(String::from(super_random_student)))
    };

    let reset = {
        let selected_student = selected_student.clone();
        Callback::from(move |_| selected_student.set(String::from(DEFAULT)))
    };

    let history = use_history().unwrap();
    let see_students = Callback::once(move |_| history.push(Route::Students));

    html! {
        <div>
            <h1>{ "Random Hat 🎩" } </h1>
            <hr/>
            <p> { "A random name generator for selecting FASLIP nominees each week."} </p>
            <h3> { "Click the big hat icon to (psuedo) randomly select a speaker for next week!" } </h3>
            <button onclick={select_random_student}> <h1> { "🎩" } </h1> </button>
            <h1> {selected_student.to_string()}</h1>
            <button onclick={reset}>{"Reset"}</button>
            <hr/>
            <button onclick={see_students}>{"See FASLIP students"}</button>
        </div>
    }
}
