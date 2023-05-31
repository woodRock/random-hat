use reqwest;
use reqwest::header::{USER_AGENT, ACCEPT_LANGUAGE, ACCEPT_ENCODING, ACCEPT, REFERER};
use serde::{Deserialize,Serialize};
use scraper::{Selector};
use scraper::Html as ScraperHtml;

#[derive(Clone, PartialEq,Debug, Deserialize, Serialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub topic: String,
    pub supervisors: Vec<String>,
    pub start_date: String,
    pub scholarship: String 
}

pub async fn scrape_students() -> Vec<Student> {
    let url = "https://ecs.wgtn.ac.nz/Groups/ECRG/FASLIP_Team";
    let mut fetched_students: Vec<Student> = vec![];
    let client = reqwest::Client::new();
    let response: String = client
        .get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/109.0")
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.5")
        .header(ACCEPT_ENCODING, "gzip, deflate, br")
        .header(ACCEPT, "	text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
        .header(REFERER, "http://www.google.com/")
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

pub fn write_students_to_json(students: Vec<Student>) {
    let students_json = serde_json::to_string(&students).unwrap();
    std::fs::write("src/models/names.json", students_json).expect("Unable to write file");
}

pub fn read_students_from_json() -> Vec<Student> {
    let students_json = include_str!("../src/models/names.json");
    let students: Vec<Student> = serde_json::from_str(students_json).expect("failed to parse JSON");
    students
}

// Main function that scrapes stduents from web and writes them to JSON file.
# [tokio::main] 
pub async fn main() {
    let students = scrape_students().await;
    write_students_to_json(students);
}