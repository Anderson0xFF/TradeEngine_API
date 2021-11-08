use rocket::{serde::json::Json, Route};

use crate::models::{book::book_service, order::Order};

pub fn book_controller() -> Vec<Route> {
    routes![find_order_by_id, post_order, buy_all, get_sell]
}

#[get("/api/order/<id>")]
fn find_order_by_id(id: u32) -> Json<Option<&'static Order>> {
    let order = book_service::find_order_by_id(id);
    match order {
        Some((order, _)) => Json(Some(order)),
        None => Json(None),
    }
}

#[post("/api/order", data = "<order>", format = "json")]
fn post_order(order: Json<Order>) {
    book_service::add_order(order.into_inner());
}

#[get("/api/order/buy")]
fn buy_all() -> Json<Vec<Order>> {
    Json(book_service::buy_orders_sort())
}

#[get("/api/order/sell")]
pub fn get_sell() -> Json<Vec<Order>> {
    Json(book_service::sell_orders_sort())
}
