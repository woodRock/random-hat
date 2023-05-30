// Students - `student.rs`
// =====================
// This file defines the Student struct and a function to read the students from a JSON file.
use serde::Deserialize;
use reqwasm::http::Request;
use scraper::{Selector};
use scraper::Html as ScraperHtml;

#[derive(Clone, PartialEq,Debug, Deserialize)]
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

pub async fn scrape_students() -> Vec<Student> {
    let url = "https://ecs.wgtn.ac.nz/Groups/ECRG/FASLIP_Team#Current_Students";
    let mut fetched_students: Vec<Student> = vec![];
    let response: String = Request::get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let html_doc = ScraperHtml::parse_document(&response);
    let ul_selector = Selector::parse("#Current_Students + ul").unwrap();
    let ul = html_doc.select(&ul_selector).next().unwrap();
    let mut id = 0;
    for li in ul.select(&Selector::parse("li").unwrap()) {
        let student = Student {
            id: id,
            name: li.select(&Selector::parse("a").unwrap()).next().unwrap().inner_html(),
            topic: li.inner_html().split(":").nth(1).unwrap().split(".").next().unwrap().to_string(),
            supervisors: li.select(&Selector::parse("a").unwrap()).skip(1).map(|a| a.inner_html()).collect(),
            start_date: li.inner_html().split(":").nth(1).unwrap().split(".").nth(2).unwrap().to_string(),
            scholarship: li.inner_html().split(":").nth(1).unwrap().split(".").nth(3).unwrap().to_string()
        };
        fetched_students.push(student);
        id += 1;
    }
    fetched_students
}