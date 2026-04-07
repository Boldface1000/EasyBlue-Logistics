// 1. Declare the sub-module (the actual local_storage.rs file)
pub mod local_storage;

// 2. Re-export for cleaner access
// This allows you to call 'offline::LocalStore' instead of 'offline::local_storage::LocalStore'
pub use local_storage::LocalStore;

/* Lead Developer Note: 
   [2026-04-06] Persistence Registry:
   If we add 'sqlite_cache.rs' or 'indexed_db.rs' later for 
   offline map tiles on the Oppo A12, we will register them here.
*/