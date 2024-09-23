use rocket::{get, post, put};
use rocket_dyn_templates::{context, Template};

use crate::repositories::TodoRepositories;

// TODO一覧画面
#[get("/")]
pub fn index() -> Template {
    let connection = &mut TodoRepositories::establish_connection();
    match TodoRepositories::show_all(connection) {
        Ok(todos) => Template::render("index", context! { todos }),
        Err(_) => Template::render(
            "error",
            context! { message: "データベースエラーが発生しました。" },
        ),
    }
}

// TODO詳細画面
#[get("/detail/<id>")]
pub fn get_todo(id: i32) -> String {
    let response = format!("Hello {}", id);
    response
}

// TODO追加画面
#[get("/add")]
pub fn add() -> String {
    let response = format!("Hello ");
    response
}

// TODO追加処理
#[post("/add/<id>")]
pub fn add_todo(id: i32) -> String {
    let response = format!("Hello {}", id);
    response
}

// TODO削除処理
#[get("/delete/<id>")]
pub fn delete(id: i32) -> String {
    let response = format!("Hello {}", id);
    response
}

// TODO更新画面
#[get("/update/<id>")]
pub fn update(id: i32) -> String {
    let response = format!("Hello {}", id);
    response
}

// TODO更新処理
#[put("/update/<id>")]
pub fn update_todo(id: i32) -> String {
    let response = format!("Hello {}", id);
    response
}
