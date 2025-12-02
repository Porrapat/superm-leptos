use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

mod api;
mod components;
mod models;

use components::{Landing, Products};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="wrapper-gray">
                <div class="container">
                    <nav class="navbar">
                        <A attr:class="logo" href="/">"SuperM"</A>
                        <div class="nav-wrapper">
                            <ul class="nav">
                                <li class="nav-item">
                                    <A href="/">"Home"</A>
                                </li>
                                <li class="nav-item">
                                    <A href="/products">"Products"</A>
                                </li>
                            </ul>
                        </div>
                    </nav>
                </div>
            </div>
            <div class="container page-wrapper">
                <Routes fallback=|| view! { <h1>"Page not found"</h1> }>
                    <Route path=path!("") view=Landing/>
                    <Route path=path!("products") view=Products/>
                </Routes>
            </div>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
