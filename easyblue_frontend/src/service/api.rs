use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

// --- 1. DATA STRUCTURES: LOGISTICS ---
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: Option<i32>,
    pub pickup_location: String,
    pub dropoff_location: String,
    pub status: String,
    pub price: f64,
}

// --- 2. DATA STRUCTURES: IDENTITY & AUTH ---
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub role: Option<String>,
    pub message: String,
}

#[derive(Serialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub message: String,
}

// --- 3. THE UNIFIED API ENGINE ---
pub struct ApiService;

impl ApiService {
    // Update this to your production URL or your local network IP
    const BASE_URL: &'static str = "http://127.0.0.1:5000/api";
    
    // SECTION: GOOGLE IDENTITY BRIDGE
    
    /// Generates the URL that triggers the Google OAuth flow on the Flask backend
    pub fn get_google_auth_url() -> String {
        format!("{}/auth/google/login", Self::BASE_URL)
    }


    // ==========================================
    // SECTION: IDENTITY ENGINE (AUTH)
    // ==========================================

    /// Trigger the 6-digit PIN Email for New Users
    pub async fn request_signup_pin(data: SignupRequest) -> Result<AuthResponse, String> {
        let url = format!("{}/auth/signup-request", Self::BASE_URL);
        
        let response = Request::post(&url)
            .json(&data)
            .map_err(|_| "Data Serialization Error")?
            .send()
            .await
            .map_err(|_| "EasyBlue Backend Unreachable")?;

        response.json::<AuthResponse>().await.map_err(|_| "Parsing Error".to_string())
    }

    /// Verify PIN & Finalize Account Creation
    pub async fn verify_pin(email: &str, pin: &str) -> Result<AuthResponse, String> {
        let url = format!("{}/auth/verify", Self::BASE_URL);
        let body = serde_json::json!({ "email": email, "pin": pin });

        let response = Request::post(&url)
            .json(&body)
            .map_err(|_| "JSON Error")?
            .send()
            .await
            .map_err(|_| "Network Error")?;

        response.json::<AuthResponse>().await.map_err(|_| "Verification Response Error".to_string())
    }

    /// Request a Recovery PIN for Password Reset
    pub async fn request_password_reset_pin(email: &str) -> Result<AuthResponse, String> {
        let url = format!("{}/auth/reset-request", Self::BASE_URL);
        let body = serde_json::json!({ "email": email });

        let response = Request::post(&url)
            .json(&body)
            .map_err(|_| "JSON Error")?
            .send()
            .await
            .map_err(|_| "Network Error")?;

        response.json::<AuthResponse>().await.map_err(|_| "Reset Request Error".to_string())
    }

    // ==========================================
    // SECTION: LOGISTICS ENGINE (ORDERS)
    // ==========================================

    pub async fn get_orders() -> Result<Vec<Order>, String> {
        let url = format!("{}/orders", Self::BASE_URL);
        let response = Request::get(&url).send().await.map_err(|_| "Network Error")?;

        if response.ok() {
            response.json::<Vec<Order>>().await.map_err(|_| "Data Parsing Error".to_string())
        } else {
            Err("Failed to fetch orders".to_string())
        }
    }

    pub async fn create_order(new_order: Order) -> Result<Order, String> {
        let url = format!("{}/orders/create", Self::BASE_URL);
        let response = Request::post(&url)
            .json(&new_order)
            .map_err(|_| "Serialization Error")?
            .send()
            .await
            .map_err(|_| "Backend is unreachable")?;

        if response.ok() {
            response.json::<Order>().await.map_err(|_| "Read Error".to_string())
        } else {
            let error: ApiError = response.json().await.map_err(|_| "Unknown Error")?;
            Err(error.message)
        }
    }
}
