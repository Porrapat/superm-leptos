use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::models::User;

#[component]
pub fn Profile<F>(user: Option<User>, on_user_logout: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    let navigate = use_navigate();
    
    // If no user, redirect to login
    if user.is_none() {
        navigate("/login", Default::default());
    }
    
    let handle_logout = move |_| {
        on_user_logout.clone()();
        navigate("/login", Default::default());
    };
    
    view! {
        <div class="profile-wrapper">
            <h1>"Profile"</h1>
            <p class="text-dimmed">
                "You are logged in as "
                <strong>{user.as_ref().map(|u| u.username.clone()).unwrap_or_default()}</strong>
                "."
            </p>
            <div class="profile-buttons">
                <input
                    type="button"
                    value="Logout"
                    class="btn"
                    on:click=handle_logout
                />
            </div>
        </div>
    }
}
