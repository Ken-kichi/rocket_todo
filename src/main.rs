use rocket::{catch, catchers, http::Status, response::status};
use rocket::{
    fs::{relative, FileServer},
    routes,
};
use rocket_dyn_templates::Template;
use rocket_todo::routes::*;
use std::env;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _port = env::var("PORT").unwrap_or_else(|_| "8000".to_string()); // デフォルトポート8000
    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                error,
                get_detail,
                add,
                add_todo,
                delete_todo,
                update,
                update_todo
            ],
        )
        .register("/", catchers![unprocessable_entity])
        .launch()
        .await?;
    Ok(())
}

#[catch(422)]
fn unprocessable_entity() -> status::Custom<String> {
    status::Custom(
        Status::UnprocessableEntity,
        "リクエストの処理に失敗しました".to_string(),
    )
}
