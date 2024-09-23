use std::io::{stdin, Read};

use rocket_todo::repositories::TodoRepositories;

fn main() {
    let connection = &mut TodoRepositories::establish_connection();

    let mut title = String::new();
    let mut completed = String::new();

    println!("タイトルをどうしますか？");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("\nOk! Let's write {title} (Press {EOF} when finished)\n",);
    stdin().read_to_string(&mut completed).unwrap();

    let completed = match completed.trim() {
        "true" => true,
        "false" => false,
        _ => false,
    };

    let todo = TodoRepositories::create(connection, &title.to_string(), &completed);
    println!("{:?}", todo)
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";
