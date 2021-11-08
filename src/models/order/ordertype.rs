use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum OrderType {
    CREATE = 1,
    DELETE
}
