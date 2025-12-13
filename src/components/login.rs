use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::JsCast;
use crate::api::login;

#[component]
pub fn Login<F>(on_user_login: F) -> impl IntoView 
where
    F: Fn(crate::models::User) + 'static + Clone,
{
    let (error_message, set_error_message) = signal(String::new());
    let (is_pending, set_is_pending) = signal(false);
    let navigate = use_navigate();
    
    let handle_login = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_error_message.set(String::new());
        set_is_pending.set(true);
        
        let form = ev.target().unwrap().dyn_into::<web_sys::HtmlFormElement>().unwrap();
        let form_data = web_sys::FormData::new_with_form(&form).unwrap();
        
        let email = form_data.get("email").as_string().unwrap_or_default();
        let password = form_data.get("password").as_string().unwrap_or_default();
        
        let on_user_login_clone = on_user_login.clone();
        let navigate_clone = navigate.clone();
        
        spawn_local(async move {
            match login(email, password).await {
                Ok(user) => {
                    on_user_login_clone(user);
                    navigate_clone("/profile", Default::default());
                }
                Err(err) => {
                    set_error_message.set(err);
                }
            }
            set_is_pending.set(false);
        });
    };

    view! {
        <div class="profile-wrapper">
            <h1>"Login"</h1>
            <p class="text-dimmed">
                "Login using test@example.com and any password."
            </p>
            <form on:submit=handle_login>
                <label class="label">
                    "Email"<span class="required">"*"</span>":"
                </label>
                <input
                    name="email"
                    type="email"
                    class="input"
                    placeholder="Email"
                    autocomplete="email"
                    disabled=move || is_pending.get()
                />
                
                <label class="label">
                    "Password"<span class="required">"*"</span>":"
                </label>
                <input
                    name="password"
                    type="password"
                    class="input"
                    placeholder="Password"
                    autocomplete="current-password"
                    disabled=move || is_pending.get()
                />
                
                <p class="login-error">{move || error_message.get()}</p>
                
                <div class="form-buttons">
                    <input
                        type="submit"
                        value="Login"
                        class="btn"
                        disabled=move || is_pending.get()
                    />
                </div>
            </form>
        </div>
    }
}
