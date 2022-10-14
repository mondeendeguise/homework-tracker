pub mod model;
use model::database::{ Grade, Task };

use time::macros::datetime;
use url::Url;

fn main() {
    let mut tasks = Vec::new();

    tasks.push(
        Task::new(
            tasks.len(),
            String::from("Wash the dishes"),
            Some(String::from("Just wash them, that's all")),
            Some(Url::parse("https://en.wikipedia.org/wiki/Wikipedia:No_climbing_the_Reichstag_dressed_as_Spider-Man")),
            Some(Grade {points: 100, score: 75}),
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
