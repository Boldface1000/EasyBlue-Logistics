use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;
use crate::services::api::ApiService; // Ensures we can call the Auth URL logic

#[component]
pub fn Login(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let email = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());

    render! {
        div { class: "signup-container",
            h2 { class: "role-title", "Welcome Back" }
            p { class: "role-description", "Log in to manage your EasyBlue deliveries." }

            // --- MANUAL LOGIN FIELDS ---
            div { class: "input-group",
                input { 
                    type: "email", 
                    placeholder: "Email Address",
                    oninput: move |e| email.set(e.value.clone())
                }
                input { 
                    type: "password", 
                    placeholder: "Password",
                    oninput: move |e| password.set(e.value.clone())
                }
            }

            // --- THE FORGOT PASSWORD BRIDGE ---
            div { class: "flex-row-between",
                label { class: "remember-me",
                    input { type: "checkbox" }
                    span { " Remember Me" }
                }
                button { 
                    class: "link-btn", 
                    onclick: move |_| { nav.push(Route::PasswordReset {}); },
                    "Forgot Password?" 
                }
            }

            button { 
                class: "btn-primary-flex", 
                onclick: |_| { /* Manual Login logic will go here */ },
                "Login" 
            }

            div { class: "divider", span { "OR" } }
            
            // --- UPDATED SOCIAL AUTH: FUNCTIONAL REDIRECT ---
            a { 
                class: "google-btn", 
                href: "{ApiService::get_google_auth_url()}", // Links to Flask backend
                style: "text-decoration: none; display: flex; align-items: center; justify-content: center;",
                img { src: "/images/google_icon.png", width: "20", style: "margin-right: 10px;" }
                span { "Continue with Google" }
            }

            // --- REDIRECT TO SIGNUP ---
            p { class: "footer-text",
                "Don't have an account? "
                span { 
                    class: "link-text", 
                    style: "cursor: pointer; color: var(--red-orange);",
                    onclick: move |_| { nav.push(Route::GetStarted {}); },
                    "Sign Up" 
                }
            }
        }
    }
}
