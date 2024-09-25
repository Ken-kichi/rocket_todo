use crate::{
    models::{NewTodo, Todo},
    schema::todos::{self},
};
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::form::Form;
use std::env;

pub struct TodoRepositories;

impl TodoRepositories {
    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("エラーを起こしているURL{}", database_url))
    }

    pub fn show_all(conn: &mut SqliteConnection) -> Result<Vec<Todo>, String> {
        Ok(todos::table
            .load(conn)
            .expect("TODOの一覧を取得できませんでした。"))
    }

    pub fn show(conn: &mut SqliteConnection, id: i32) -> Result<Todo, String> {
        todos::table
            .find(id)
            .first(conn)
            .map_err(|_| "TODOの詳細を取得できませんでした。".to_string())
    }

    pub fn create(
        conn: &mut SqliteConnection,
        new_title: &String,
        new_description: &String,
        completed: &bool,
    ) -> Result<usize, String> {
        let new_todo = NewTodo {
            title: new_title.to_string(),
            description: new_description.to_string(),
            completed: *completed,
        };

        Ok(diesel::insert_into(todos::table)
            .values(&new_todo)
            .execute(conn)
            .expect("Error saving new todo"))
    }

    pub fn update(conn: &mut SqliteConnection, update_todo: Form<Todo>) -> Result<usize, String> {
        Ok(diesel::update(todos::table.find(update_todo.id))
            .set((
                todos::title.eq(update_todo.title.clone()),
                todos::description.eq(update_todo.description.clone()),
                todos::completed.eq(update_todo.completed),
            ))
            .execute(conn)
            .expect("Error updating todo"))
    }

    pub fn delete(conn: &mut SqliteConnection, id: i32) -> Result<usize, String> {
        let deleted = diesel::delete(todos::table.find(id))
            .execute(conn)
            .expect("Error deleting posts");
        Ok(deleted)
    }
}
