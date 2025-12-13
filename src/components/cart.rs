use leptos::prelude::*;
use leptos_router::components::*;
use crate::models::{CartItem, User};

#[component]
pub fn Cart(
    cart: Vec<CartItem>,
    user: Option<User>,
    on_add_product: impl Fn(u32) + 'static + Copy + Send,
    on_remove_product: impl Fn(u32) + 'static + Copy + Send,
) -> impl IntoView {
    let cart_is_empty = cart.is_empty();
    
    let cart_sum = cart.iter()
        .map(|item| item.final_price * item.quantity)
        .sum::<u32>();
    
    let (email, set_email) = signal(user.as_ref().map(|u| u.email.clone()).unwrap_or_default());
    let (show_success, set_show_success) = signal(false);
    
    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        // Dummy API call - just show success message
        set_show_success.set(true);
        
        // Hide success message after 3 seconds
        set_timeout(
            move || {
                set_show_success.set(false);
            },
            std::time::Duration::from_secs(3),
        );
    };

    view! {
        <div class="cart-wrapper">
            <h1>"Your cart"</h1>
            {move || {
                if cart_is_empty {
                    view! {
                        <p>
                            "Your cart is empty. Add a product from the "
                            <A href="/products">"products"</A>
                            " page."
                        </p>
                    }.into_any()
                } else {
                    let cart_clone = cart.clone();
                    view! {
                        <>
                            {cart_clone.into_iter().map(|item| {
                                let item_id_add = item.id;
                                let item_id_remove = item.id;
                                view! {
                                    <div class="cart-product">
                                        <img
                                            src={item.thumbnail.clone()}
                                            alt={item.name.clone()}
                                            width="126"
                                            height="84"
                                        />
                                        <div class="cart-product-details">
                                            <div class="cart-product-name">
                                                <p>{item.name.clone()}</p>
                                                <ul class="cart-buttons">
                                                    <li>{item.quantity}</li>
                                                    <li>
                                                        <button on:click=move |_| on_add_product(item_id_add)>"+"</button>
                                                    </li>
                                                    <li>
                                                        <button on:click=move |_| on_remove_product(item_id_remove)>"-"</button>
                                                    </li>
                                                </ul>
                                            </div>
                                            <p>{format!("${:.2}", item.final_price as f32 / 100.0)}</p>
                                        </div>
                                        <p>{format!("${:.2}", (item.final_price * item.quantity) as f32 / 100.0)}</p>
                                    </div>
                                }
                            }).collect_view()}

                            <div class="cart-total">
                                <h2>"Your total price"</h2>
                                <p class="cart-total-value">{format!("${:.2}", cart_sum as f32 / 100.0)}</p>
                            </div>

                            {move || if show_success.get() {
                                view! {
                                    <div style="padding: 1rem; background-color: #4caf50; color: white; border-radius: 4px; margin-bottom: 1rem;">
                                        "✓ Order placed successfully! (Demo - No actual API call made)"
                                    </div>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}

                            <form on:submit=on_submit>
                                <label class="label">
                                    "Email"<span class="required">"*"</span>":"
                                </label>
                                <input
                                    type="email"
                                    class="input"
                                    placeholder="Enter your email"
                                    prop:value=move || email.get()
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                    required
                                />
                                <p class="text-dimmed cart-notice">
                                    "Enter your email and then click on pay and your products will be delivered to you the same day!"
                                </p>
                                <p class="cart-notice cart-warning">
                                    "This is a demo, so the form does not submit any data."
                                </p>
                                <div class="cart-button-wrapper">
                                    <input type="submit" value="Pay" class="btn" />
                                </div>
                            </form>
                        </>
                    }.into_any()
                }
            }}
        </div>
    }
}
