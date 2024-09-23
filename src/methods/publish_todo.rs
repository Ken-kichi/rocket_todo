use rocket_todo::repositories::TodoRepositories;
use std::env::args;

fn main() {
    let id: i32 = args()
        .nth(1)
        .expect("publish_todo required a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut TodoRepositories::establish_connection();

    let post_todo = TodoRepositories::update(connection, id);
    println!("{}", post_todo)
}
