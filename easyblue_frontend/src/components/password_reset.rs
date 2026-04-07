use dioxus::prelude::*;

// Define the States of our Machine
#[derive(PartialEq)]
enum ResetState {
    RequestCode,
    VerifyCode,
    NewPassword,
    Complete,
}

#[component]
pub fn PasswordReset(cx: Scope) -> Element {
    let current_state = use_state(cx, || ResetState::RequestCode);
    let email = use_state(cx, || String::new());
    let pin = use_state(cx, || vec![""; 6]);

    render! {
        div { class: "signup-container",
            match *current_state.get() {
                // STEP 1: Input Email
                ResetState::RequestCode => rsx! {
                    h2 { class: "role-title", "Forgot Password?" }
                    p { class: "role-description", "Enter your email to receive a recovery code." }
                    input { 
                        type: "email", 
                        placeholder: "yourname@email.com",
                        oninput: move |e| email.set(e.value.clone())
                    }
                    button { 
                        class: "btn-primary-flex", 
                        onclick: |_| current_state.set(ResetState::VerifyCode),
                        "Send Code" 
                    }
                },

                // STEP 2: The 6-Digit PIN Gate
                ResetState::VerifyCode => rsx! {
                    h2 { class: "role-title", "Check Your Email" }
                    p { class: "role-description", "We sent a 6-digit code to {email}" }
                    div { class: "pin-container",
                        for i in 0..6 {
                            input { 
                                class: "pin-input", 
                                type: "number", 
                                maxlength: "1",
                                oninput: |_| { /* Focus logic handled by CSS/JS */ }
                            }
                        }
                    }
                    button { 
                        class: "btn-primary-flex", 
                        onclick: |_| current_state.set(ResetState::NewPassword),
                        "Verify Code" 
                    }
                },

                // STEP 3: Create New Credentials
                ResetState::NewPassword => rsx! {
                    h2 { class: "role-title", "New Password" }
                    input { type: "password", placeholder: "New Password" }
                    input { type: "password", placeholder: "Confirm New Password" }
                    button { 
                        class: "btn-primary-flex", 
                        onclick: |_| current_state.set(ResetState::Complete),
                        "Update Password" 
                    }
                },

                // STEP 4: Success State
                ResetState::Complete => rsx! {
                    div { class: "glass-card", style: "text-align: center",
                        h2 { "Success!" }
                        p { "Your password has been updated. You can now log in." }
                        button { class: "btn-primary-flex", "Go to Login" }
                    }
                }
            }
        }
    }
}
