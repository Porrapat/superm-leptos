use crate::models::{Product, ProductDetails, User};
use wasm_bindgen::JsCast;
use web_sys::{Request, RequestInit, RequestMode, Response};

const API_KEY: &str = "";
const BASE_URL: &str = "https://superm-api.porrapat.com/";

pub async fn fetch_products() -> Result<Vec<Product>, String> {
    let url = format!("{}products-list", BASE_URL);
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    
    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;
    
    request
        .headers()
        .set("apikey", API_KEY)
        .map_err(|e| format!("Failed to set header: {:?}", e))?;
    
    let window = web_sys::window().ok_or("No window found")?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Failed to fetch: {:?}", e))?;
    
    let resp: Response = resp_value.dyn_into().map_err(|_| "Response is not a Response")?;
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json().map_err(|e| format!("Failed to get JSON: {:?}", e))?
    )
    .await
    .map_err(|e| format!("Failed to parse JSON: {:?}", e))?;
    
    serde_wasm_bindgen::from_value(json)
        .map_err(|e| format!("Failed to deserialize: {}", e))
}

pub async fn fetch_product_details(id: u32) -> Result<ProductDetails, String> {
    let url = format!("{}products/id/{}", BASE_URL, id);
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    
    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;
    
    request
        .headers()
        .set("apikey", API_KEY)
        .map_err(|e| format!("Failed to set header: {:?}", e))?;
    
    let window = web_sys::window().ok_or("No window found")?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Failed to fetch: {:?}", e))?;
    
    let resp: Response = resp_value.dyn_into().map_err(|_| "Response is not a Response")?;
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json().map_err(|e| format!("Failed to get JSON: {:?}", e))?
    )
    .await
    .map_err(|e| format!("Failed to parse JSON: {:?}", e))?;
    
    let products: Vec<ProductDetails> = serde_wasm_bindgen::from_value(json)
        .map_err(|e| format!("Failed to deserialize: {}", e))?;
    
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
