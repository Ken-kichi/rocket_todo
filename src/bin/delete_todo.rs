use std::env::args;
use rocket_todo::repositories::TodoRepositories;

fn main() {
    let target = args().nth(1).expect("Expected a target to match against");
    let id = target.parse::<i32>().unwrap();

    let connection = &mut TodoRepositories::establish_connection();
    let num_deleted = TodoRepositories::delete(connection, id);

    println!("{}", num_deleted)
}
