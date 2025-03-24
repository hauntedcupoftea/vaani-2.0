#[macro_use]
extern crate rocket;
mod train;

#[get("/")]
fn index() -> &'static str {
    train::train()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
