use wasm_bindgen::prelude::*;
use web_sys::{window, ServiceWorkerContainer};

pub struct ServiceWorkerManager;

impl ServiceWorkerManager {
    /// Checks if a Service Worker is currently controlling the page
    pub fn is_active() -> bool {
        if let Some(win) = window() {
            if let Ok(nav) = win.navigator().service_worker() {
                return nav.controller().is_some();
            }
        }
        false
    }

    /// Forces the Service Worker to update (Useful for pushing new EasyBlue features)
    pub async fn update_app() -> Result<(), String> {
        let win = window().ok_or("No Window")?;
        let nav = win.navigator();
        let container: ServiceWorkerContainer = nav.service_worker();
        
        let registration = wasm_bindgen_futures::JsFuture::from(container.get_registration())
            .await
            .map_err(|_| "Failed to get SW registration")?;

        if !registration.is_undefined() {
            let reg: web_sys::ServiceWorkerRegistration = registration.into();
            let _ = reg.update().map_err(|_| "Update failed")?;
            return Ok(());
        }
        
        Err("No Service Worker found".to_string())
    }

    /// Returns the "Offline Status" to display in the UI
    pub fn get_status_label() -> &'static str {
        if Self::is_active() {
            "Ready for Offline Use"
        } else {
            "Connecting..."
        }
    }
}
