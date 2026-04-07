use dioxus::prelude::*;
use crate::services::api::ApiService; // Import the API Engine for the Google URL

#[component]
pub fn GetStarted(cx: Scope) -> Element {
    // 1. Core State Management
    let step = use_state(cx, || 1); // 1: Role, 2: Info, 3: PIN
    let user_role = use_state(cx, || "customer");
    let name = use_state(cx, || String::new());
    let email = use_state(cx, || String::new());

    // Calculate progress percentage for the bar
    let progress = match *step.get() {
        1 => "33%",
        2 => "66%",
        _ => "100%",
    };

    render! {
        div { class: "signup-container",
            // --- PROGRESS BAR ---
            div { class: "progress-container",
                div { class: "progress-bar", style: "width: {progress}" }
            }

            match *step.get() {
                1 => rsx! {
                    // STEP 1: ROLE SELECTION
                    h2 { class: "role-title", "Choose Your Role" }
                    div { class: "role-card", onclick: |_| { user_role.set("rider"); step.set(2); },
                        div { class: "role-icon-circle", "🛵" }
                        div { class: "role-text-content",
                            h3 { "I am a Rider" }
                            p { "Manage company vehicles & deliver." }
                        }
                    }
                    div { class: "role-card", onclick: |_| { user_role.set("vendor"); step.set(2); },
                        div { class: "role-icon-circle", "🏢" }
                        div { class: "role-text-content",
                            h3 { "I am a Vendor" }
                            p { "Scale your business with EasyBlue." }
                        }
                    }
                    div { class: "role-card", onclick: |_| { user_role.set("customer"); step.set(2); },
                        div { class: "role-icon-circle", "📦" }
                        div { class: "role-text-content",
                            h3 { "Standard Customer" }
                            p { "Simple, fast door-to-door delivery." }
                        }
                    }
                },

                2 => rsx! {
                    // STEP 2: BASIC INFO
                    h2 { class: "role-title", "Tell Us About You" }
                    p { class: "role-description", "Registering as a {user_role.get().to_uppercase()}" }
                    
                    input { 
                        type: "text", 
                        placeholder: "Full Name",
                        oninput: move |e| name.set(e.value.clone())
                    }
                    input { 
                        type: "email", 
                        placeholder: "Email Address",
                        oninput: move |e| email.set(e.value.clone())
                    }
                    
                    button { 
                        class: "btn-primary-flex", 
                        onclick: |_| step.set(3), 
                        "Send Verification PIN" 
                    }

                    // --- FUNCTIONAL GOOGLE REDIRECT ---
                    a { 
                        class: "google-btn", 
                        href: "{ApiService::get_google_auth_url()}",
                        style: "text-decoration: none; display: flex; align-items: center; justify-content: center;",
                        img { src: "/images/google_icon.png", width: "20", style: "margin-right: 10px;" }
                        span { "Continue with Google" }
                    }

                    button { 
                        class: "btn-secondary", 
                        onclick: |_| step.set(1), 
                        "Back" 
                    }
                },

                3 => rsx! {
                    // STEP 3: 6-DIGIT PIN VERIFICATION
                    h2 { class: "role-title", "Verify Email" }
                    p { class: "role-description", "We sent a code to {email}" }
                    
                    div { class: "pin-container",
                        for i in 0..6 {
                            input { 
                                class: "pin-input", 
                                type: "number", 
                                maxlength: "1" 
                            }
                        }
                    }

                    button { 
                        class: "btn-primary-flex", 
                        onclick: |_| { /* Logic to finalize with api::verify_pin */ }, 
                        "Complete Registration" 
                    }
                    
                    p { class: "info-text", "Didn't get it? Resend in 59s" }
                },

                _ => rsx! { div { "Redirecting..." } }
            }
        }
    }
}
