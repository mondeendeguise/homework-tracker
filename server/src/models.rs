#[allow(unused)]

use url::{ParseError, Url};

pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub url: Option<Vec<Url>>,
    pub grade: Option<&'a [u32; 2]>,
    // pub date_due: Option<&'a str>,
    // pub date_done: Option<&'a str>,
}

pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<Vec<Url>>,
    pub grade: Option<[u32; 2]>,
    // pub date_due: Option<String>,
    // pub date_done: Option<String>,
    pub is_complete: bool,
    pub is_removed: bool,
}

pub struct NewGroup<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub url: Option<Vec<Url>>,
    // pub date_due: Option<String>,
    // pub date_done: Option<String>,
}

pub struct Group {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<Vec<Url>>,
    // pub date_due: Option<String>,
    // pub date_done: Option<String>,
    pub tasks: Vec<Task>,
    pub is_complete: bool,
    pub is_removed: bool,
}

pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub timezone: &'a str,
}

pub struct User {
    pub id: u32,
    pub name: String,
    // pub email: String,
    // pub password: String,
    // pub timezone: String,
    pub groups: Vec<Group>,
}
