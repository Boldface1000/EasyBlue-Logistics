// 1. Declare the sub-modules (the actual .rs files in this folder)
pub mod api;       // Handles ALL requests (Auth, Orders, Reset) to Flask
// pub mod location; // Reserved for future GPS tracking for Riders

// 2. Re-export common functions for easier access
// This allows you to call 'services::ApiService' directly
pub use api::ApiService;

/* Timeline Note: 
   [2026-04-06] Service Registry:
   Unified Auth and API into a single module to reduce cross-crate overhead.
   Ready to add 'location' and 'payment' mods as the dashboard scales.
*/
