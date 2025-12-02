use leptos::prelude::*;
use leptos_router::components::*;
use crate::models::Product as ProductModel;
use crate::components::price::Price;

#[component]
pub fn Product(details: ProductModel) -> impl IntoView {
    let product_url = format!("/products/{}", details.id);
    
    view! {
        <div class="product">
            <A href=product_url>
                <img
                    class="product-image"
                    width="272"
                    height="300"
                    src=details.thumbnail.clone()
                    alt=details.name.clone()
                />
                <p class="product-name">{details.name.clone()}</p>
                <div class="product-price">
                    <Price
                        final_price=details.final_price
                        original_price=details.original_price
                    />
                </div>
            </A>
        </div>
    }
}
