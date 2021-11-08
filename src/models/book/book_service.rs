use std::{fs::File, io::Write};

use rocket::serde::json::serde_json;

use crate::models::{
    self,
    order::{orderside::OrderSide, Order},
    trade_history::{OrderStatus, TradeHistory},
};

use super::Book;

fn book() -> &'static mut Book {
    unsafe {
        match &mut models::DB {
            Some(book) => book,
            None => {
                models::initialize_db();
                book()
            }
        }
    }
}

pub fn add_order(order: Order) {
    match order.type_op() {
        models::order::ordertype::OrderType::CREATE => book().orders_mut().push(order),
        models::order::ordertype::OrderType::DELETE => remove(order.order_id()),
    };
}

fn decrement_amount(order_id: u32, amount: f32) {
    match book().find_order_by_id_as_mut(order_id) {
        Some((order, _)) => {
            let amount = order.amount() - amount;
            order.set_amount(amount);
        }
        None => return,
    }
}

pub fn find_order_by_id_as_mut(order_id: u32) -> Option<(&'static mut Order, usize)> {
    let mut index = 0;
    for value in &book().orders {
        if value.order_id() == order_id {
            return Some((&mut book().orders[index], index));
        }
        index += 1;
    }
    None
}

fn buy_all() -> Vec<Order> {
    let mut buffer = Vec::new();
    for value in book().orders.clone() {
        if value.side().eq(&OrderSide::BUY) {
            buffer.push(value);
        }
    }
    buffer
}

fn sell_all() -> Vec<Order> {
    let mut buffer = Vec::new();
    for value in book().orders.clone() {
        if value.side().eq(&OrderSide::SELL) {
            buffer.push(value);
        }
    }
    buffer
}

pub fn sell_orders_sort() -> Vec<Order> {
    let mut buffer = book().sell_all();
    buffer.sort_by(|x, y| x.limit_price().partial_cmp(&y.limit_price()).unwrap());
    buffer
}

pub fn buy_orders_sort() -> Vec<Order> {
    let mut buffer = book().buy_all();
    buffer.sort_by(|x, y| x.limit_price().partial_cmp(&y.limit_price()).unwrap());
    buffer.reverse();
    buffer
}

pub fn save(path: &str) {
    let mut file = File::create(&path).unwrap();
    let string = serde_json::to_string_pretty(&book().orders).unwrap();
    file.write_all(string.as_bytes()).unwrap();
    file.flush().unwrap();
}

pub fn find_order_by_id(order_id: u32) -> Option<(&'static Order, usize)> {
    let mut index = 0;
    for value in &book().orders {
        if value.order_id() == order_id {
            return Some((&book().orders[index], index));
        }
        index += 1;
    }
    None
}

pub fn remove(order_id: u32) {
    match book().find_order_by_id(order_id) {
        Some((_, index)) => book().orders.remove(index),
        None => todo!(),
    };
}

pub fn process_orders(tradehistory: &mut TradeHistory) {
    for buy in book().buy_orders_sort() {
        for sell in book().sell_orders_sort() {
            if (buy.limit_price() >= sell.limit_price()) && (buy.account_id() != sell.account_id())
            {
                if buy.amount() > sell.amount() {
                    book().decrement_amount(buy.order_id(), sell.amount());
                    tradehistory.add_history(OrderStatus::PARTIAL, (sell.clone(), buy.clone()));
                    book().remove(sell.order_id());
                } else if sell.amount() > buy.amount() {
                    book().decrement_amount(sell.order_id(), buy.amount());
                    tradehistory.add_history(OrderStatus::PARTIAL, (sell.clone(), buy.clone()));
                    book().remove(buy.order_id());
                } else {
                    tradehistory.add_history(OrderStatus::COMPLETE, (sell.clone(), buy.clone()));
                    book().remove(buy.order_id());
                    book().remove(sell.order_id());
                    break;
                }
            }
        }
    }
}
