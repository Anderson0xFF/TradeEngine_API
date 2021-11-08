use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum OrderSide {
    BUY,
    SELL,
}
