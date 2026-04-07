#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// --- STEP 1: MODULE REGISTRY ---
mod components; 
mod services;   
mod offline;    

// --- STEP 2: IMPORTS ---
use components::welcome::Welcome; 
use components::signup::GetStarted; // Added missing semicolon here
use components::login::Login;          
use components::password_reset::PasswordReset;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // These pages sit OUTSIDE the NavBar (Full screen onboarding)
    #[route("/")]
    Welcome {},

    #[route("/signup")]
    GetStarted {}, 

    #[route("/login")]
    Login {},                          

    #[route("/forgot-password")]
    PasswordReset {},                   

    // These pages sit INSIDE the NavBar layout
    #[layout(NavBar)]
        #[route("/dashboard-rider")]
        RiderDashboard {},
        #[route("/dashboard-vendor")]
        VendorDashboard {},
        #[route("/dashboard-customer")]
        CustomerDashboard {},
        
        // Added placeholders to match your NavBar links
        #[route("/book")]
        BookingForm {},
        #[route("/map")]
        LiveMap {},
        #[route("/profile")]
        UserProfile {},
    #[end_layout]
}

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

// --- STEP 3: THE NAVIGATION BAR ---
#[component]
fn NavBar(cx: Scope) -> Element {
    render! {
        div { class: "app-container",
            Outlet::<Route> {}

            nav { class: "bottom-nav",
                Link { to: Route::Welcome {}, class: "nav-item",
                    div { class: "nav-icon", "🏠" }
                    span { "Home" }
                }
                // Updated to point to specific Routes defined in the enum
                Link { to: Route::BookingForm {}, class: "nav-item",
                    div { class: "nav-icon", "📦" }
                    span { "Book" }
                }
                Link { to: Route::LiveMap {}, class: "nav-item",
                    div { class: "nav-icon", "📍" }
                    span { "Track" }
                }
                Link { to: Route::UserProfile {}, class: "nav-item",
                    div { class: "nav-icon", "👤" }
                    span { "Profile" }
                }
            }
        }
    }
}

// --- STEP 4: PLACEHOLDERS ---
#[component] fn RiderDashboard(cx: Scope) -> Element { render! { div { "Rider Dashboard" } } }
#[component] fn VendorDashboard(cx: Scope) -> Element { render! { div { "Vendor Dashboard" } } }
#[component] fn CustomerDashboard(cx: Scope) -> Element { render! { div { "Customer Dashboard" } } }
#[component] fn BookingForm(cx: Scope) -> Element { render! { div { "Booking Screen" } } }
#[component] fn LiveMap(cx: Scope) -> Element { render! { div { "Real-time Tracking" } } }
#[component] fn UserProfile(cx: Scope) -> Element { render! { div { "Your Profile" } } }
