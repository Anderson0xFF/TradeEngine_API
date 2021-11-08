
#[macro_use]
extern crate rocket;

mod controllers;
mod models;

#[launch]
fn rocket() -> _ {
    models::initialize_db();
    let controllers = controllers::routers();

    rocket::build().mount("/", controllers)
}
