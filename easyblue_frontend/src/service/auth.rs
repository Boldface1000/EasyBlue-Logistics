use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

// 1. Data structure for the User's Session
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserSession {
    pub token: String,
    pub user_id: i32,
    pub role: String, // "vendor", "rider", or "customer"
    pub is_authenticated: bool,
}

pub struct AuthService;

impl AuthService {
    const STORAGE_KEY: &'static str = "easyblue_auth_session";

    /// Saves the login session to the phone's local storage
    pub fn save_session(session: UserSession) -> Result<(), String> {
        LocalStorage::set(Self::STORAGE_KEY, session)
            .map_err(|_| "Failed to save session to device".to_string())
    }

    /// Retrieves the session (checks if the user is already logged in)
    pub fn get_session() -> Option<UserSession> {
        LocalStorage::get(Self::STORAGE_KEY).ok()
    }

    /// Clears the session (Logout)
    pub fn logout() {
        LocalStorage::delete(Self::STORAGE_KEY);
    }

    /// Returns the JWT token for API headers
    pub fn get_token() -> Option<String> {
        Self::get_session().map(|s| s.token)
    }

    /// Checks if the user has the required permission (e.g., Rider vs Vendor)
    pub fn has_role(role: &str) -> bool {
        match Self::get_session() {
            Some(s) => s.role == role,
            None => false,
        }
    }
}
