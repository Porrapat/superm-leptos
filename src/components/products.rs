use leptos::prelude::*;
use crate::api::fetch_products;
use crate::components::product::Product;

#[component]
pub fn Products() -> impl IntoView {
    let products_resource = LocalResource::new(|| fetch_products());
    
    let (query, set_query) = signal(String::new());
    
    let filtered_products = Memo::new(move |_| {
        let q = query.get().trim().to_lowercase();
        
        match products_resource.get() {
            Some(result) => {
                match result.as_ref() {
                    Ok(products) => {
                        if q.is_empty() {
                            products.clone()
                        } else {
                            products
                                .iter()
                                .filter(|p| p.name.to_lowercase().contains(&q))
                                .cloned()
                                .collect()
                        }
                    }
                    Err(_) => vec![],
                }
            }
            _ => vec![],
        }
    });
    
    let handle_search_change = move |ev| {
        let value = event_target_value(&ev);
        set_query.set(value);
    };
    
    let handle_reset = move |_| {
        set_query.set(String::new());
    };
    
    view! {
        <div>
            <div class="products-title">
                <h1>"Products"</h1>
                <input
                    type="search"
                    class="search"
                    prop:value=query
                    placeholder="Search products"
                    on:input=handle_search_change
                />
            </div>
            
            <Suspense fallback=move || view! { <div class="loading">"Loading products..."</div> }>
                {move || {
                    products_resource.get().map(|result| {
                        match result.as_ref() {
                            Ok(_) => {
                                let products = filtered_products.get();
                                if products.is_empty() && !query.get().is_empty() {
                                    view! {
                                        <div>
                                            <div class="products-not-found">
                                                <div>
                                                    <h2>"No products found!"</h2>
                                                    <p>
                                                        "Your search \""<strong>{query.get()}</strong>"\" was not found in our store."
                                                    </p>
                                                    <button
                                                        class="btn btn-dimmed"
                                                        type="button"
                                                        on:click=handle_reset
                                                    >
                                                        "Reset search"
                                                    </button>
                                                </div>
                                            </div>
                                            <div class="products-grid"></div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="products-grid">
                                            <For
                                                each=move || filtered_products.get()
                                                key=|product| product.id
                                                children=move |product| {
                                                    view! {
                                                        <Product details=product />
                                                    }
                                                }
                                            />
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(e) => view! {
                                <div class="loading">
                                    <p>"Error loading products: " {e.clone()}</p>
                                </div>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
