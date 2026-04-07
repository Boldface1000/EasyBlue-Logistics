#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// --- STEP 1: LINK TO YOUR EXTERNAL FOLDERS ---
mod components; // Looks for src/components/mod.rs
mod services;   // Looks for src/services/mod.rs
mod offline;    // Looks for src/offline/mod.rs

// --- STEP 2: IMPORT THE HIGH-END WELCOME PAGE ---
// This replaces the old placeholder Welcome component at the bottom
use components::welcome::Welcome; 

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Welcome {}, // Now calling the component from components/welcome.rs
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
// (We keep these here until we move them to their own files)
#[component] fn BookingForm(cx: Scope) -> Element { render! { div { "Booking Screen" } } }
#[component] fn LiveMap(cx: Scope) -> Element { render! { div { "Real-time Tracking" } } }
#[component] fn UserProfile(cx: Scope) -> Element { render! { div { "Your Profile" } } }
