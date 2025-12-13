use leptos::prelude::*;
use web_sys::window;
use crate::models::User;

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
                        <a href="/">"Home"</a>
                    </li>

                    <li class="nav-item">
                        {move || {
                            if user().is_some() {
                                view! { <a href="/profile">"Profile"</a> }
                            } else {
                                view! { <a href="/login">"Login"</a> }
                            }
                        }}
                    </li>

                    <li class="nav-item">
                        <a href="/products">"Products"</a>
                    </li>
                </ul>

                <a href="/cart" class="btn btn-nav">
                    {move || format!("Cart ({})", cart_count())}
                </a>
            </nav>
        </div>
    }
}
