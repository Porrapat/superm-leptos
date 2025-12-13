use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use crate::api::fetch_product_details;
use crate::components::price::Price;
use crate::models::Product;

#[component]
pub fn ProductDetails(
    on_add_product: impl Fn(Product) + 'static + Copy + Send,
) -> impl IntoView {
    let params = use_params_map();
    
    let product_resource = LocalResource::new(move || async move {
        let id = params.read().get("id").and_then(|id| id.parse::<u32>().ok());
        match id {
            Some(id) => fetch_product_details(id).await,
            None => Err("Invalid product ID".to_string()),
        }
    });

    view! {
        <Suspense fallback=move || view! { <p>"Loading product details..."</p> }>
            {move || {
                product_resource.get().map(|result| {
                    match result {
                        Ok(details) => {
                            let product = Product {
                                id: details.id,
                                name: details.name.clone(),
                                thumbnail: details.thumbnail.clone(),
                                final_price: details.final_price,
                                original_price: details.original_price,
                            };
                            
                            view! {
                                <>
                                    <A href="/products" attr:class="back">
                                        "‹ Back to products"
                                    </A>
                                    <div class="details">
                                        <div>
                                            <img
                                                src=details.thumbnail.clone()
                                                alt=details.name.clone()
                                                width="612"
                                                height="408"
                                                class="details-image"
                                            />

                                            <h2>"Product details"</h2>
                                            <table class="nutrition">
                                                <thead>
                                                    <tr>
                                                        <th>"Nutrient"</th>
                                                        <th>"Value"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    <tr>
                                                        <td>"Protein"</td>
                                                        <td>{format!("{} g", details.nutrition.protein)}</td>
                                                    </tr>
                                                    <tr>
                                                        <td>"Carbohydrates"</td>
                                                        <td>{format!("{} g", details.nutrition.carbs)}</td>
                                                    </tr>
                                                    <tr>
                                                        <td>"Fat"</td>
                                                        <td>{format!("{} g", details.nutrition.fat)}</td>
                                                    </tr>
                                                </tbody>
                                            </table>
                                        </div>
                                        <div>
                                            <h1 class="details-name">{details.name.clone()}</h1>
                                            <p class="details-price">
                                                <Price
                                                    final_price=details.final_price
                                                    original_price=details.original_price
                                                />
                                            </p>
                                            <p
                                                class="text-dimmed"
                                                inner_html=details.description.clone()
                                            ></p>
                                            <div class="details-a2c">
                                                <button
                                                    class="btn btn-block"
                                                    on:click=move |_| on_add_product(product.clone())
                                                >
                                                    "Add to cart"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </>
                            }.into_any()
                        }
                        Err(e) => {
                            view! {
                                <div>
                                    <p class="error">"Error loading product: " {e}</p>
                                    <A href="/products" attr:class="back">
                                        "‹ Back to products"
                                    </A>
                                </div>
                            }.into_any()
                        }
                    }
                })
            }}
        </Suspense>
    }
}
