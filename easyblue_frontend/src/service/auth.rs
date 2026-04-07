use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserSession {
    pub token: String,
    pub user_id: i32,
    pub role: String, // "vendor", "rider", or "customer"
    pub is_authenticated: bool,
}

pub struct AuthService;

impl AuthService {
    const STORAGE_KEY: &'static str = "easy_auth_v1"; // Versioned key for safety

    pub fn save_session(session: UserSession) -> Result<(), String> {
        LocalStorage::set(Self::STORAGE_KEY, session)
            .map_err(|_| "Security Storage Access Denied".to_string())
    }

    pub fn get_session() -> Option<UserSession> {
        LocalStorage::get(Self::STORAGE_KEY).ok()
    }

    pub fn logout() {
        LocalStorage::delete(Self::STORAGE_KEY);
    }

    /// NEW: Quick check for the Main Router
    pub fn is_logged_in() -> bool {
        Self::get_session().map(|s| s.is_authenticated).unwrap_or(false)
    }

    pub fn get_token() -> Option<String> {
        Self::get_session().map(|s| s.token)
    }

    pub fn has_role(role: &str) -> bool {
        Self::get_session().map(|s| s.role == role).unwrap_or(false)
    }
}
