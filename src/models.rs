use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub thumbnail: String,
    pub final_price: u32,
    pub original_price: u32,
}
