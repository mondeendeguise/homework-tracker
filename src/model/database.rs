use time::PrimitiveDateTime;
use url::{ParseError, Url};

#[derive(Debug)]
pub struct Grade {
    pub received: u32,
    pub possible: u32,
}

impl Grade {
    pub fn score(&self) -> f64 {
        f64::from(self.received) / f64::from(self.possible)
    }
}

#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<Result<Url, ParseError>>,
    pub grade: Option<Grade>,
    pub date_due: Option<PrimitiveDateTime>,
    pub date_done: Option<PrimitiveDateTime>,
}

impl Task {
    pub fn new(
        id: usize,
        title: String,
        description: Option<String>,
        url: Option<Result<Url, ParseError>>,
        grade: Option<Grade>,
        date_due: Option<PrimitiveDateTime>,
        date_done: Option<PrimitiveDateTime>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            url,
            grade,
            date_due,
            date_done,
        }
    }

    pub fn status() -> [bool; 3] {
        // let current_time = std::time::SystemTime::now();
        // I'm very frustrated as I cannot figure out getting
        // the current date & time without needing to import
        // another got damn crate >:(
        todo!();
    }
}

pub struct Group {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<Result<Url, ParseError>>,
    pub items: Vec<Task>,
}

impl Group {
    pub fn new(
        id: usize,
        title: String,
        description: Option<String>,
        url: Option<Result<Url, ParseError>>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            url,
            items: Vec::new(),
        }
    }

    pub fn total_points() -> u32 {
        todo!();
        // for i in
    }

    pub fn priority(id: usize) -> f64 {
        todo!()
    }
}

