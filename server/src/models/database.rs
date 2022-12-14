#![allow(unused_variables)]

use time::PrimitiveDateTime;
use url::{ParseError, Url};

// {{{ Grade 
#[derive(Debug)]
pub struct Grade {
    pub received: u32,
    pub possible: u32,
}

impl Grade {
    pub fn new(possible: u32) -> Self {
        Self {
            received: 0,
            possible,
        }
    }

    pub fn score(&self) -> f64 {
        f64::from(self.received) / f64::from(self.possible)
    }
}
// }}} Grade

// {{{ Task
#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<Result<Url, ParseError>>,
    pub grade: Option<Grade>,
    pub date_due: Option<PrimitiveDateTime>,
    pub date_done: Option<PrimitiveDateTime>,
    pub is_complete: bool,
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
            is_complete: false,
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
// }}} Task

// {{{ Group 
#[derive(Debug)]
pub struct Group {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<Result<Url, ParseError>>,
    pub tasks: Vec<Task>,
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
            tasks: Vec::new(),
        }
    }

    pub fn total_points(&self) -> u32 {
        let mut total: u32 = 0;
        for task in &self.tasks {
            total += task.grade.as_ref().unwrap().received;
        }
        return total;
    }

    // TODO: priority function based on other grades & impact
    pub fn priority(id: usize) -> f64 {
        todo!()
    }
}
// }}} Group

// {{{ User 
#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub email: String,
    // TODO: password hashing, authentication
    pub password: String,
    pub groups: Vec<Group>,
}

impl User {
    pub fn new(id: usize, name: String, email: String, password: String) -> Self {
        Self {
            id,
            name,
            email,
            password,
            groups: Vec::new(),
        }
    }

    // TODO: user authentication
    pub fn authenticate(email: String, password: String) {
        todo!();
    }
}
// }}} User

// {{{ Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_score() {
        let score = (Grade {
            received: 75,
            possible: 100,
        }).score();
        assert_eq!(score, 0.75);
    }
}
// }}} Tests
