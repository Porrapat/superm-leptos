use crate::models::{Product, ProductDetails, User};
use gloo_net::http::Request;

const API_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImJud2R4ZHVqZ3pyemd0Zmt5c215Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MjYyMjUxMDIsImV4cCI6MjA0MTgwMTEwMn0.D0nuB2PYrkIVuIsz3R2JqJLJYHmr8gXChAiZrTGMiHk";
const BASE_URL: &str = "https://bnwdxdujgzrzgtfkysmy.supabase.co/rest/v1/";

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
    let url = format!("{}products?id=eq.{}", BASE_URL, id);
    
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
