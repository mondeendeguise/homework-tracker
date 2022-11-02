use homework_tracker::models::database::{Grade, Task};

use time::macros::datetime;
use url::Url;

#[test]
fn grade_test() {
    assert_eq!(
        Grade {
            received: 75,
            possible: 100
        }
        .score(),
        0.75
    );
}

#[test]
fn user_test() {
    let mut tasks = Vec::new();
    tasks.push(Task::new(
        tasks.len(),
        String::from("Wash the dishes"),
        Some(String::from("Just wash them, that's all")),
        Some(Url::parse("https://en.wikipedia.org/wiki/Wikipedia:No_climbing_the_Reichstag_dressed_as_Spider-Man")),
        Some(Grade {received: 75, possible: 100}),
        Some(datetime!(2023-04-03 13:20)),
        Some(datetime!(2023-04-30 23:59)),
    ));

    tasks.push(Task::new(
        tasks.len(),
        String::from("Do the laundry"),
        None,
        Some(Url::parse(
            "https://www.github.com/mondeendeguise/homework-tracker/",
        )),
        None,
        None,
        None,
    ));
    
    assert_eq!(tasks[0].grade.as_ref().unwrap().score(), 0.75);
    assert_eq!(tasks[0].id, 0);
    assert_eq!(tasks[1].id, 1);
}
