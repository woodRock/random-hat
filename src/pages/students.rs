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
            <h3>{"FASLIP - Current Students"}</h3>
            { students }
            <ReturnHome />
        </div>
    }
}

#[function_component(Students)]
pub fn students() -> Html {

    let students: Vec<Student> = serde_json::from_str(STUDENTS_JSON).expect("failed to parse JSON");
    println!("{:?}", students);

    html! {
        <>
            <StudentsList students={students} />
        </>
    }
}

// let videos = vec![
//     Video {
//         id: 1,
//         title: "Building and breaking things".to_string(),
//         speaker: "John Doe".to_string(),
//         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//     },
//     Video {
//         id: 2,
//         title: "The development process".to_string(),
//         speaker: "Jane Smith".to_string(),
//         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//     },
//     Video {
//         id: 3,
//         title: "The Web 7.0".to_string(),
//         speaker: "Matt Miller".to_string(),
//         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//     },
//     Video {
//         id: 4,
//         title: "Mouseless development".to_string(),
//         speaker: "Tom Jerry".to_string(),
//         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//     },
// ];

// let videos = videos.iter().map(|video| html! {
//     <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
// }).collect::<Html>();