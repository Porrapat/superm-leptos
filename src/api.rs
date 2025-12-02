use crate::models::Product;
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
