use rocket::{Route, serde::json::Json};

use crate::models::{book::book_service, order::Order};


pub fn routers() -> Vec<Route>{
    routes!
    [
        get_buy_sort,
        post_buy,
    ]

}

#[get("/api/order/buy")]
pub fn get_buy_sort() -> Json<Vec<Order>> {
    Json(book_service::buy_orders_sort())
}

#[post("/api/order/buy", data = "<order>", format = "json")]
pub fn post_buy(order: Json<Order>) {
    book_service::add_order(order.into_inner())
}

