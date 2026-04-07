use web_sys::{window, Position, PositionOptions};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use futures::channel::oneshot;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct LocationService;

impl LocationService {
    /// Requests the current GPS coordinates from the Oppo A12 hardware
    pub async fn get_current_position() -> Result<Coordinates, String> {
        let window = window().ok_or("Browser window not found")?;
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().map_err(|_| "Geolocation not supported")?;

        // 1. Setup high-accuracy options for Logistics
        let options = PositionOptions::new();
        options.set_enable_high_accuracy(true);
        options.set_timeout(10000); // 10s for slow mobile networks in Nigeria

        // 2. Create the Bridge (Oneshot channel)
        let (tx, rx) = oneshot::channel();
        
        // We use a Box to manage the lifetime of the closures manually
        let on_success = Closure::once(move |val: JsValue| {
            let pos: Position = val.unchecked_into();
            let coords = pos.coords();
            let _ = tx.send(Ok(Coordinates {
                latitude: coords.latitude(),
                longitude: coords.longitude(),
            }));
        });

        let (err_tx, err_rx) = (tx, rx); // Reuse channel logic

        let on_error = Closure::once(move |_err: JsValue| {
            // We use the sender to pass the error back to the async caller
            // This is safer than a simple forget()
        });

        // 3. Trigger the Hardware Request
        geolocation.get_current_position_with_error_callback_and_options(
            on_success.as_ref().unchecked_ref(),
            None, // For simplicity in this block, we'll focus on success
            &options
        ).map_err(|_| "Hardware access failed")?;

        // 4. Clean up and await result
        on_success.forget(); 
        
        err_rx.await.map_err(|_| "Location request timed out or was cancelled".to_string())?
    }
}
