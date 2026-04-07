use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

// 1. Define what we want to "remember"
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppSettings {
    pub notifications_enabled: bool,
    pub dark_mode: bool,
    pub last_synced: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DraftOrder {
    pub pickup_address: String,
    pub dropoff_address: String,
    pub package_type: String,
}

pub struct LocalStore;

impl LocalStore {
    const SETTINGS_KEY: &'static str = "easyblue_settings";
    const DRAFT_KEY: &'static str = "easyblue_draft_order";

    /// --- Settings Management ---
    pub fn save_settings(settings: AppSettings) {
        let _ = LocalStorage::set(Self::SETTINGS_KEY, settings);
    }

    pub fn get_settings() -> AppSettings {
        LocalStorage::get(Self::SETTINGS_KEY).unwrap_or_default()
    }

    /// --- Draft Management (Crucial for unstable mobile data) ---
    pub fn save_draft(order: DraftOrder) {
        let _ = LocalStorage::set(Self::DRAFT_KEY, order);
    }

    pub fn get_draft() -> Option<DraftOrder> {
        LocalStorage::get(Self::DRAFT_KEY).ok()
    }

    pub fn clear_draft() {
        LocalStorage::delete(Self::DRAFT_KEY);
    }

    /// --- Generic Storage (For flexibility) ---
    pub fn set_item<T: Serialize>(key: &str, value: T) -> Result<(), String> {
        LocalStorage::set(key, value).map_err(|_| "Storage Full or Access Denied".to_string())
    }

    pub fn get_item<T: for<'de> Deserialize<'de>>(key: &str) -> Option<T> {
        LocalStorage::get(key).ok()
    }
}
