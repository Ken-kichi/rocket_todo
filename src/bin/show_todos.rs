use rocket_todo::models::Todo;
use rocket_todo::repositories::TodoRepositories;
fn main() {
    let connection = &mut TodoRepositories::establish_connection();
    let results: Vec<Todo> = TodoRepositories::show_all(connection);
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{} {}", post.title, post.completed);
        println!("-----------\n");
    }
}
