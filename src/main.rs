use rocket::{fs::relative, fs::FileServer, routes};
use rocket_dyn_templates::Template;
use rocket_todo::routes::*;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .mount("/", routes![
            index,
            error,
            get_detail,
            add,
            add_todo
            ])
            .launch()
        .await?;
    Ok(())
}
