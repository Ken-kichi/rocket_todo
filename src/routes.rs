use crate::{models::{NewTodo,DeleteForm}, repositories::TodoRepositories};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::{get, post, put,delete};
use rocket_dyn_templates::{context, Template};

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
pub fn get_detail(id: i32) -> Template {
    let connection = &mut TodoRepositories::establish_connection();
    match TodoRepositories::show(connection,id){
        Ok(todo)=>Template::render("detail",context!{todo:todo}),
        Err(_)=>Template::render(
            "error",
            context!{message:"データベースエラーが発生しました。"}
        )
    }
}

// TODO追加画面
#[get("/add?<message>")]
pub fn add(message: Option<String>) -> Template {
    Template::render("add", context! {message})
}

// TODO追加処理
#[post(
    "/add",
    format = "application/x-www-form-urlencoded",
    data = "<new_todo>"
)]
pub fn add_todo(new_todo: Form<NewTodo>) -> Redirect {
    let connection = &mut TodoRepositories::establish_connection();
    match TodoRepositories::create(connection, &new_todo.title, &new_todo.completed) {
        Ok(_todo) => Redirect::to("/"),
        Err(e) => Redirect::to(format!("/error?message={}", e)),
    }
}

// TODO削除処理
#[post("/delete/<id>",format = "application/x-www-form-urlencoded", data = "<_form>")]
pub fn delete_todo(id: i32,_form: Form<DeleteForm>) -> Redirect {
    let connection = &mut TodoRepositories::establish_connection();
    match TodoRepositories::delete(connection,id){
        Err(e) => Redirect::to(format!("/error?message={}", e)),
        Ok(0) => Redirect::to("/"), // 0件削除された場合の処理
        Ok(_) => Redirect::to("/"), // 1件以上削除された場合の処理
    }
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

#[get("/error?<message>")]
pub fn error(message: Option<String>) -> Template {
    Template::render("error", context! {message})
}
