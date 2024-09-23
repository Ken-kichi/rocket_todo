use rocket_todo::models::*;
use rocket_todo::repositories::TodoRepositories;
use std::env::args;

fn main() {
    let todo_id = args()
        .nth(1)
        .expect("get_todo requires a todo id")
        .parse::<i32>()
        .expect("invalid ID");

    let connection = &mut TodoRepositories::establish_connection();

    let todo: Option<Todo> = TodoRepositories::show(connection, todo_id);

    println!("{:?}", todo)
}
