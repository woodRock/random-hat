// Students - `student.rs`
use crate::models::student::{Student,scrape_students};
use crate::components::return_home::ReturnHome;

use yew::prelude::*;
use yew::{html};

#[derive(Properties, PartialEq)]
struct StudentsListProps {
    students: Vec<Student>,
}

#[function_component(StudentsList)]
fn student_list(StudentsListProps { students }: &StudentsListProps) -> Html {
    let students = students.iter().map(|student| html! {
        <>
            <p key={student.id}>{format!("{}: {}", student.name, student.topic)}</p>
            // <img class="small profile" src={
            //     format!("https://ecs.wgtn.ac.nz/foswiki/pub/Main/Grad{}/{}.jpg", 
            //     student.name.replace(" ", ""),
            //     student.name.replace(" ", "")
            //     )
            // }/>
        </>
    }).collect::<Html>();

    html! {
        <div>
            { students }
        </div>
    }
}

#[function_component(Students)]
pub fn students() -> Html {

    let students = use_state(|| vec![]);
    {
        let students = students.clone();
        use_effect_with_deps(move |_| {
            let students = students.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_students = scrape_students().await;
                students.set(fetched_students);
            });
            || () 
        }, ());
    }

    html! {
        <>    
            <header>       
                <h1>{"FASLIP - Current Students"}</h1>
            </header>
            <hr/>
            <div id="description"> 
                {" List is from the "} 
                <a href="https://ecs.wgtn.ac.nz/Groups/ECRG/FASLIP_Team#Current_Students">{ "ECRG FASLIP" }</a> 
                { " website."}    
            </div>
            <hr/>
            <StudentsList students={(*students).clone()} />
            <hr/>
            <footer>
                <ReturnHome />
            </footer>
        </>
    }
}