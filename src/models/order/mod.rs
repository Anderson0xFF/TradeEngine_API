#![allow(proc_macro_derive_resolution_fallback, unused_imports)]

use rocket::{
    data::{FromData, Outcome},
    http::Status,
    request::{self, FromRequest},
    serde::{Deserialize, Serialize},
    Request,
};

use self::{orderside::OrderSide, ordertype::OrderType};
pub mod orderside;
pub mod ordertype;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    type_op: OrderType,
    account_id: u32,
    amount: f32,
    order_id: u32,
    pair: String,
    limit_price: f32,
    side: OrderSide,
}

impl Order {
    pub fn new(
        type_op: OrderType,
        account_id: u32,
        amount: f32,
        order_id: u32,
        pair: String,
        limit_price: f32,
        side: OrderSide,
    ) -> Self {
        Self {
            type_op,
            account_id,
            amount,
            order_id,
            pair,
            limit_price,
            side,
        }
    }

    /// Get a reference to the order's side.
    pub fn side(&self) -> &OrderSide {
        &self.side
    }

    /// Get a reference to the order's limit price.
    pub fn limit_price(&self) -> f32 {
        self.limit_price
    }

    // pub fn from_value(input: &Value) -> Result<Order, &str> {
    //     let type_op = match OrderType::from_str(input["type_op"].as_str().unwrap()) {
    //         Ok(e) => e,
    //         Err(e) => return Err(e),
    //     };
    //     let account_id = match input["account_id"].as_i64() {
    //         Some(id) => id as u32,
    //         None => return Err("account_id not found."),
    //     };
    //     let amount = match input["amount"].as_f64() {
    //         Some(id) => id as f32,
    //         None => return Err("amount not found."),
    //     };

    //     let order_id = match input["order_id"].as_i64() {
    //         Some(id) => id as u32,
    //         None => return Err("order_id not found."),
    //     };

    //     let limit_price = match input["limit_price"].as_f64() {
    //         Some(id) => id as f32,
    //         None => return Err("limit_price not found."),
    //     };

    //     let side = match OrderSide::from_str(input["side"].as_str().unwrap()) {
    //         Ok(e) => e,
    //         Err(e) => return Err(e),
    //     };

    //     let mut pair = input["pair"].to_string();
    //     pair.remove(0);
    //     pair.remove(pair.len() - 1);

    //     Ok(Order::new(
    //         type_op,
    //         account_id,
    //         amount,
    //         order_id,
    //         pair,
    //         limit_price,
    //         side,
    //     ))
    // }

    /// Get a reference to the order's order id.
    pub fn order_id(&self) -> u32 {
        self.order_id
    }

    /// Get a reference to the order's type op.
    pub fn type_op(&self) -> &OrderType {
        &self.type_op
    }

    /// Get a reference to the order's account id.
    pub fn account_id(&self) -> u32 {
        self.account_id
    }

    /// Get a reference to the order's amount.
    pub fn amount(&self) -> f32 {
        self.amount
    }

    /// Get a mutable reference to the order's amount.
    pub fn amount_mut(&mut self) -> &mut f32 {
        &mut self.amount
    }

    /// Set the order's amount.
    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount;
    }
}
