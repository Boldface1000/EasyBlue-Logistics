use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

// 1. Define the Data Structures (Must match your Flask Models)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: Option<i32>,
    pub pickup_location: String,
    pub dropoff_location: String,
    pub status: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub message: String,
}

// 2. The API Service Logic
pub struct ApiService;

impl ApiService {
    // The base URL of your Flask backend (update this to your Termux or Server IP)
    const BASE_URL: &'static str = "http://127.0.0.1:5000/api";

    /// Fetches all active orders for the current user
    pub async fn get_orders() -> Result<Vec<Order>, String> {
        let url = format!("{}/orders", Self::BASE_URL);
        
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|_| "Network Error: Could not reach EasyBlue Backend")?;

        if response.ok() {
            response.json::<Vec<Order>>().await.map_err(|_| "Data Parsing Error".to_string())
        } else {
            Err("Failed to fetch orders".to_string())
        }
    }

    /// Sends a new simultaneous pickup/dropoff booking to Flask
    pub async fn create_order(new_order: Order) -> Result<Order, String> {
        let url = format!("{}/orders/create", Self::BASE_URL);
        
        let response = Request::post(&url)
            .json(&new_order)
            .map_err(|_| "Serialization Error")?
            .send()
            .await
            .map_err(|_| "Backend is unreachable")?;

        if response.ok() {
            response.json::<Order>().await.map_err(|_| "Success, but failed to read response".to_string())
        } else {
            let error: ApiError = response.json().await.map_err(|_| "Unknown Error")?;
            Err(error.message)
        }
    }
}
