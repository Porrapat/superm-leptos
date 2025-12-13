use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

mod api;
mod components;
mod models;

use components::{Landing, Products, ProductDetails, Cart, Login, Navbar, Profile};
use models::{User, CartItem, Product};

#[component]
fn App() -> impl IntoView {
    let (user, set_user) = signal::<Option<User>>(None);
    let (cart, set_cart) = signal::<Vec<CartItem>>(Vec::new());
    
    let handle_user_login = move |new_user: User| {
        set_user.set(Some(new_user));
    };
    
    let handle_user_logout = move || {
        set_user.set(None);
    };
    
    let handle_add_product = move |product: Product| {
        set_cart.update(|cart| {
            if let Some(item) = cart.iter_mut().find(|item| item.id == product.id) {
                item.quantity += 1;
            } else {
                cart.push(CartItem {
                    id: product.id,
                    name: product.name,
                    thumbnail: product.thumbnail,
                    final_price: product.final_price,
                    quantity: 1,
                });
            }
        });
    };
    
    let handle_increment_item = move |product_id: u32| {
        set_cart.update(|cart| {
            if let Some(item) = cart.iter_mut().find(|item| item.id == product_id) {
                item.quantity += 1;
            }
        });
    };
    
    let handle_decrement_item = move |product_id: u32| {
        set_cart.update(|cart| {
            if let Some(item) = cart.iter_mut().find(|item| item.id == product_id) {
                if item.quantity > 1 {
                    item.quantity -= 1;
                } else {
                    cart.retain(|item| item.id != product_id);
                }
            }
        });
    };
    
    let cart_count = move || {
        cart.get().iter().map(|item| item.quantity).sum::<u32>()
    };
    
    view! {
        <Router>
            <div class="wrapper-gray">
                <div class="container">
                    <Navbar user=move || user.get() cart_count=cart_count />
                </div>
            </div>
            <div class="container page-wrapper">
                <Routes fallback=|| view! { <h1>"Page not found"</h1> }>
                    <Route path=path!("") view=Landing/>
                    <Route path=path!("products") view=move || view! { <Products on_add_product=handle_add_product /> }/>
                    <Route path=path!("products/:id") view=move || view! { <ProductDetails on_add_product=handle_add_product /> }/>
                    <Route path=path!("cart") view=move || view! { <Cart cart=cart.get() user=user.get() on_add_product=handle_increment_item on_remove_product=handle_decrement_item /> }/>
                    <Route path=path!("login") view=move || view! { <Login on_user_login=handle_user_login /> }/>
                    <Route path=path!("profile") view=move || view! { <Profile user=user.get() on_user_logout=handle_user_logout /> }/>
                </Routes>
            </div>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
