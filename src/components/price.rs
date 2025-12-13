use leptos::prelude::*;

#[component]
pub fn Price(original_price: u32, final_price: u32) -> impl IntoView {
    let final_price_display = format!("{:.2}", final_price as f64 / 100.0);
    let original_price_display = format!("{:.2}", original_price as f64 / 100.0);
    
    view! {
        <>
            "$"{final_price_display}
            {move || {
                if final_price != original_price {
                    view! {
                        <span class="price-old">"$"{original_price_display.clone()}</span>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </>
    }
}
