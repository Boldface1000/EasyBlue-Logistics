use dioxus::prelude::*;
use dioxus_router::prelude::*; // 1. Necessary for navigation
use crate::Route; // 2. Access the Routes defined in main.rs

#[component]
pub fn Welcome(cx: Scope) -> Element {
    let nav = use_navigator(cx); // 3. The "Gearbox" for changing pages

    let slides = [
        ("RIDER", "Earn money on your own schedule. Deliver with speed.", "images/step1.png"),
        ("VENDOR", "Grow your business. We handle the heavy lifting.", "images/step2.png"),
        ("CUSTOMER", "Fast, reliable, and secure door-to-door delivery.", "images/step3.png"),
    ];

    render! {
        div { class: "welcome-screen",
            // --- THE CAROUSEL DOM ---
            div { class: "carousel-viewport",
                for (i, (title, desc, img_path)) in slides.iter().enumerate() {
                    div { 
                        class: "carousel-item",
                        key: "{i}",
                        img { class: "carousel-img", src: "{img_path}" }
                        div { class: "role-info-container",
                            h2 { class: "role-title", "{title}" }
                            p { class: "role-description", "{desc}" }
                        }
                    }
                }
            }

            // --- THE AUTH NAVIGATION (Horizontal) ---
            div { class: "auth-button-group",
                button { 
                    class: "btn-secondary",
                    // 4. Update this to your Login route once created
                    onclick: move |_| { /* nav.push(Route::Login {}); */ },
                    "Login"
                }
                button { 
                    class: "btn-primary-flex",
                    // 5. Connects directly to the Onboarding page
                    onclick: move |_| { nav.push(Route::GetStarted {}); },
                    "Get Started"
                }
            }
        }
    }
}
