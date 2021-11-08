#[macro_use]
extern crate rocket;

mod controllers;
mod models;

#[launch]
fn rocket() -> _ {
    models::initialize_db();
    rocket::build()
    .mount("/", controllers::book_controller::book_controller())
}
