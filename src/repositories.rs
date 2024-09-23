use crate::{
    models::{NewTodo, Todo},
    schema::todos,
};
use diesel::prelude::*;
use dotenvy::dotenv;
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
        Ok(todos::table.load(conn).expect("Error load todos"))
    }

    pub fn show(conn: &mut SqliteConnection, id: i32) -> Option<Todo> {
        todos::table.find(id).first(conn).optional().unwrap()
    }

    pub fn create(
        conn: &mut SqliteConnection,
        title: &String,
        completed: &bool,
    ) -> Result<usize, String> {
        let new_todo = NewTodo {
            title: title.to_string(),
            completed: *completed,
        };

        Ok(diesel::insert_into(todos::table)
            .values(&new_todo)
            .execute(conn)
            .expect("Error saving new todo"))
    }

    pub fn update(conn: &mut SqliteConnection, id: i32) -> String {
        let updated = diesel::update(todos::table.find(id))
            .set(todos::completed.eq(true))
            .execute(conn)
            .unwrap();
        match updated {
            1 => "更新できました。".to_string(),
            0 => "更新できませんでした。".to_string(),
            _ => "問題が発生しました。".to_string(),
        }
    }

    pub fn delete(conn: &mut SqliteConnection, id: i32) -> String {
        let deleted = diesel::delete(todos::table.find(id))
            .execute(conn)
            .expect("Error deleting posts");
        match deleted {
            1 => "削除できました。".to_string(),
            0 => "削除できませんでした。".to_string(),
            _ => "問題が発生しました。".to_string(),
        }
    }
}
