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

    /// Get a reference to the order's side.
    pub fn side(&self) -> &OrderSide {
        &self.side
    }

    /// Get a reference to the order's limit price.
    pub fn limit_price(&self) -> f32 {
        self.limit_price
    }

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
