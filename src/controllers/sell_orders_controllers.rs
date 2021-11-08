use rocket::{Route, serde::json::Json};

use crate::models::{book::book_service, order::Order};

pub fn routers() -> Vec<Route>{
    routes!
    [
        get_sell,
        post_sell,
    ]

}


#[get("/api/order/sell")]
pub fn get_sell() -> Json<Vec<Order>> {
    Json(book_service::sell_orders_sort())
}

#[post("/api/order/sell", data = "<order>", format = "json")]
pub fn post_sell(order: Json<Order>) {
    book_service::add_order(order.into_inner())
}

