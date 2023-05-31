// Students - `student.rs`
// =====================
// This file defines the Student struct and a function to read the students from a JSON file.
use serde::{Deserialize,Serialize};

#[derive(Clone, PartialEq,Debug, Deserialize, Serialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub topic: String,
    pub supervisors: Vec<String>,
    pub start_date: String,
    pub scholarship: String 
}

pub fn read_students_from_json() -> Vec<Student> {
    let students_json = include_str!("names.json");
    let students: Vec<Student> = serde_json::from_str(students_json).expect("failed to parse JSON");
    students
}