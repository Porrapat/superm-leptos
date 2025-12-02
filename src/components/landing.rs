use leptos::prelude::*;
use leptos_router::components::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div>
            <h1>"Online shopping simplified"</h1>
            <p class="tagline text-dimmed">
                "Order your groceries from SuperM with our easy to use app, and
                get your products delivered straight to your doorstep."
            </p>
            <A attr:class="btn" href="/products">
                "Start shopping"
            </A>
            <img
                class="landing-cover"
                width="816"
                height="380"
                src="/landing.jpg"
                alt="Display of fruits and vegetables"
            />
        </div>
    }
}
