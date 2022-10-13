mod model;

use time::PrimitiveDateTime;
use time::macros::datetime;
use url::{ Url, ParseError };

#[derive(Debug)]
pub struct Task {
    id: usize,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<Result<Url, ParseError>>,
    pub points: Option<u32>,
    pub date_due: Option<PrimitiveDateTime>,
    pub date_done: Option<PrimitiveDateTime>,
    // priority: f64,
    // late: bool,
    // completed: bool,
    // closed: bool,
}

impl Task {
    pub fn new(
        id: usize,
        title: String,
        description: Option<String>,
        url: Option<Result<Url, ParseError>>,
        points: Option<u32>,
        date_due: Option<PrimitiveDateTime>,
        date_done: Option<PrimitiveDateTime>,
    ) -> Self {
        Task {
            id,
            title,
            description,
            url,
            points,
            date_due,
            date_done,
            // priority: 0.0,
            // late: false,
            // completed: false,
            // closed: false,
        }
    }

    pub fn status() -> [bool; 3] {
        // let current_time = std::time::SystemTime::now();
        // I'm very frustrated as I cannot figure out getting
        // the current date & time without needing to import
        // another got damn crate >:(
        todo!();
    }

    pub fn priority() -> f64 {
        todo!();
    }
}

fn main() {
    let mut tasks = Vec::new();

    tasks.push(
        Task::new(
            tasks.len(),
            String::from("Wash the dishes"),
            Some(String::from("Just wash them, that's all")),
            Some(Url::parse("https://en.wikipedia.org/wiki/Wikipedia:No_climbing_the_Reichstag_dressed_as_Spider-Man")),
            Some(35),
            Some(datetime!(2023-04-03 13:20)),
            Some(datetime!(2023-04-30 23:59)),
        )
    );

    tasks.push(
        Task::new(
            tasks.len(),
            String::from("Do the laundry"),
            None,
            Some(Url::parse("https://www.github.com/mondeendeguise/homework-tracker/")),
            None,
            None,
            None,
        )
    );

    println!("{:#?}", tasks);
    tasks[0].title = String::from("Sexy sex");
    println!("{:#?}", tasks);
}
