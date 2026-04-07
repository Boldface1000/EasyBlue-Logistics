use web_sys::{window, Geolocation, Position, PositionOptions};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct LocationService;

impl LocationService {
    /// Requests the current GPS coordinates from the Oppo A12 hardware
    pub async fn get_current_position() -> Result<Coordinates, String> {
        let window = window().ok_or("No Window Found")?;
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().map_err(|_| "Geolocation Not Supported")?;

        // Set high accuracy for precise logistics tracking
        let mut options = PositionOptions::new();
        options.enable_high_accuracy(true);
        options.timeout(5000); // 5 second timeout

        // We use a Promise bridge because the Web Geolocation API is Callback-based
        let (send, recv) = futures::channel::oneshot::channel();
        
        let on_success = Closure::once(move |pos: JsValue| {
            let pos: Position = pos.into();
            let coords = pos.coords();
            let _ = send.send(Ok(Coordinates {
                latitude: coords.latitude(),
                longitude: coords.longitude(),
            }));
        });

        let on_error = Closure::once(move |_err: JsValue| {
            let _ = send.send(Err("Location Access Denied".to_string()));
        });

        geolocation.get_current_position_with_error_callback_and_options(
            on_success.as_ref().unchecked_ref(),
            Some(on_error.as_ref().unchecked_ref()),
            &options
        ).map_err(|_| "Failed to start location request")?;

        // Keep closures alive until the promise resolves
        on_success.forget();
        on_error.forget();

        recv.await.map_err(|_| "Channel Closed")?
    }
}
