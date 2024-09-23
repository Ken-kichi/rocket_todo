use rocket::get;
use rocket::routes;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", routes![index]).launch().await?;

    Ok(())
}
