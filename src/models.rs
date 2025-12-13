use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub thumbnail: String,
    pub final_price: u32,
    pub original_price: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Nutrition {
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProductDetails {
    pub id: u32,
    pub name: String,
    pub thumbnail: String,
    pub final_price: u32,
    pub original_price: u32,
    pub description: String,
    pub nutrition: Nutrition,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub id: u32,
    pub name: String,
    pub thumbnail: String,
    pub final_price: u32,
    pub quantity: u32,
}
