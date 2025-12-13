use leptos::prelude::*;
use web_sys::window;
use crate::models::User;
use leptos_router::hooks::use_location;

#[component]
pub fn Navbar<F, G>(user: F, cart_count: G) -> impl IntoView
where
    F: Fn() -> Option<User> + 'static + Send,
    G: Fn() -> u32 + 'static + Send,
{

    let (light, set_light) = signal(true);

    let toggle_theme = move |_| {
        let new_value = !light.get();
        set_light.set(new_value);

        if let Some(doc) = window().and_then(|w| w.document()) {
            if let Some(body) = doc.body() {
                body.class_list()
                    .toggle("dark")
                    .expect("toggle class failed");
            }
        }
    };

    let location = use_location();
    
    let is_active = move |path: &'static str| {
        move || {
            let current = location.pathname.get();

            let active = if path == "/" {
                current == "/"
            } else {
                current == path || current.starts_with(&format!("{}/", path))
            };

            if active { "active" } else { "" }
        }
    };

    view! {
        <div class="navbar">
            <a class="logo" href="/">
                "SuperM"
            </a>
            <nav class="nav-wrapper">
                <button class="theme-switcher" on:click=toggle_theme>
                    <img
                        src=move || if light.get() { "/light.svg" } else { "/dark.svg" }
                        width="24"
                        height="24"
                        alt=move || if light.get() { "Light theme" } else { "Dark theme" }
                    />
                </button>

                <ul class="nav">
                    <li class="nav-item">
                        <a href="/" class=is_active("/")>"Home"</a>
                    </li>

                    <li class="nav-item">
                        {move || {
                            if user().is_some() {
                                view! { <a href="/profile" class=is_active("/profile")>"Profile"</a> }
                            } else {
                                view! { <a href="/login" class=is_active("/login")>"Login"</a> }
                            }
                        }}
                    </li>

                    <li class="nav-item">
                        <a href="/products" class=is_active("/products")>"Products"</a>
                    </li>
                </ul>

                <a href="/cart" class="btn btn-nav">
                    {move || format!("Cart ({})", cart_count())}
                </a>
            </nav>
        </div>
    }
}
