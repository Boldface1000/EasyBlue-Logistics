use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

// --- 1. SESSION DATA (The Missing Piece) ---
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserSession {
    pub token: String,
    pub role: String, // "rider", "vendor", or "customer"
    pub email: String,
    pub is_authenticated: bool,
}

// --- 2. APP SETTINGS ---
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppSettings {
    pub notifications_enabled: bool,
    pub dark_mode: bool,
    pub last_synced: String,
}

// --- 3. DRAFT MANAGEMENT ---
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DraftOrder {
    pub pickup_address: String,
    pub dropoff_address: String,
    pub package_type: String,
}

pub struct LocalStore;

impl LocalStore {
    const SETTINGS_KEY: &'static str = "easy_settings";
    const DRAFT_KEY: &'static str = "easy_draft";
    const SESSION_KEY: &'static str = "easy_session"; // NEW: Security Key

    /// --- Session Management (CRITICAL) ---
    pub fn save_session(session: UserSession) {
        let _ = LocalStorage::set(Self::SESSION_KEY, session);
    }

    pub fn get_session() -> UserSession {
        LocalStorage::get(Self::SESSION_KEY).unwrap_or_default()
    }

    pub fn logout() {
        LocalStorage::delete(Self::SESSION_KEY);
    }

    /// --- Settings Management ---
    pub fn save_settings(settings: AppSettings) {
        let _ = LocalStorage::set(Self::SETTINGS_KEY, settings);
    }

    pub fn get_settings() -> AppSettings {
        LocalStorage::get(Self::SETTINGS_KEY).unwrap_or_default()
    }

    /// --- Draft Management (For unstable mobile data) ---
    pub fn save_draft(order: DraftOrder) {
        let _ = LocalStorage::set(Self::DRAFT_KEY, order);
    }

    pub fn get_draft() -> Option<DraftOrder> {
        LocalStorage::get(Self::DRAFT_KEY).ok()
    }

    pub fn clear_draft() {
        LocalStorage::delete(Self::DRAFT_KEY);
    }

    /// --- Generic Storage ---
    pub fn set_item<T: Serialize>(key: &str, value: T) -> Result<(), String> {
        LocalStorage::set(key, value).map_err(|_| "Storage Full or Access Denied".to_string())
    }

    pub fn get_item<T: for<'de> Deserialize<'de>>(key: &str) -> Option<T> {
        LocalStorage::get(key).ok()
    }
}
