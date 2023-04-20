use crate::models::student::Student;
use crate::models::student::STUDENTS_JSON;
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
        <p key={student.id}>{format!("{}: {}", student.name, student.topic)}</p>
    }).collect::<Html>();

    html! {
        <div>
            { students }
        </div>
    }
}

#[function_component(Students)]
pub fn students() -> Html {

    let students: Vec<Student> = serde_json::from_str(STUDENTS_JSON).expect("failed to parse JSON");
    println!("{:?}", students);

    html! {
        <>            
            <h3>{"FASLIP - Current Students"}</h3>
            <hr/>
            <StudentsList students={students} />
            <hr/>
            <p> 
                {" List is from the "} 
                <a href="https://ecs.wgtn.ac.nz/Groups/ECRG/FASLIP_Team#Current_Students">{ "ECRG FASLIP" }</a> 
                { " website."}    
            </p>
            <ReturnHome />
        </>
    }
}