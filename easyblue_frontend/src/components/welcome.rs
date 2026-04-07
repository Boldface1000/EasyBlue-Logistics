use dioxus::prelude::*;

#[component]
pub fn Welcome(cx: Scope) -> Element {
    // 1. Track the current slide (0, 1, or 2)
    let current_slide = use_state(cx, || 0);

    // 2. Define the slide data
    let slides = [
        ("RIDER", "Earn money on your own schedule. Deliver with speed.", "assets/images/step1.png"),
        ("VENDOR", "Grow your business. We handle the heavy lifting.", "assets/images/step2.png"),
        ("CUSTOMER", "Fast, reliable, and secure door-to-door delivery.", "assets/images/step3.png"),
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
                    onclick: |_| { /* Navigate to Login */ },
                    "Login"
                }
                button { 
                    class: "btn-primary-flex",
                    onclick: |_| { /* Trigger the Questionnaire */ },
                    "Get Started"
                }
            }
        }
    }
}
