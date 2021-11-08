use rocket::Route;

mod buy_orders_controllers;
mod sell_orders_controllers;

pub fn routers() -> Vec<Route> {
    let mut vect = Vec::new();
    vect.extend(buy_orders_controllers::routers());
    vect.extend(sell_orders_controllers::routers());

    vect
}
