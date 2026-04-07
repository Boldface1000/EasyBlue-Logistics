// 1. Declare the sub-modules (the actual .rs files in this folder)
pub mod auth;      // Handles Login, Logout, and JWT tokens
pub mod api;       // Handles GET/POST requests to your Flask backend
pub mod location;  // Handles GPS tracking for Riders

// 2. Re-export common functions for easier access
// This allows you to call 'services::login()' instead of 'services::auth::login()'
pub use auth::AuthService;
pub use api::ApiService;
pub use location::LocationService;

/* Timeline Note: 
   As the Lead Developer, you keep this file clean. 
   If you add a 'payment' service later, you just add 'pub mod payment;' here.
*/
