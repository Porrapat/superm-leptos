use crate::models::{Product, ProductDetails, User};
use reqwasm::http::Request;

const API_KEY: &str = "";
const BASE_URL: &str = "https://superm-api.porrapat.com/";

pub async fn fetch_products() -> Result<Vec<Product>, String> {
    let url = format!("{}products-list", BASE_URL);
    
    let response = Request::get(&url)
        .header("apikey", API_KEY)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch products: {}", e))?;
    
    if !response.ok() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    response
        .json::<Vec<Product>>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))
}

pub async fn fetch_product_details(id: u32) -> Result<ProductDetails, String> {
    let url = format!("{}products/id/{}", BASE_URL, id);
    
    let response = Request::get(&url)
        .header("apikey", API_KEY)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch product details: {}", e))?;
    
    if !response.ok() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    let products: Vec<ProductDetails> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    products.into_iter()
        .next()
        .ok_or_else(|| "Product not found".to_string())
}

pub async fn login(email: String, password: String) -> Result<User, String> {
    if email == "test@example.com" && !password.is_empty() {
        Ok(User {
            email,
            username: "testuser".to_string(),
        })
    } else {
        Err("Invalid email or password".to_string())
    }
}
