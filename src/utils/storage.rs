#[cfg(feature = "desktop")]
use dioxus_logger::tracing::{debug, error, info};
use serde::{Serialize, de::DeserializeOwned};

/// Error types for storage operations.
#[derive(Debug)]
#[allow(dead_code)]
pub enum StorageError {
    /// Error accessing storage
    #[cfg(target_arch = "wasm32")]
    AccessError,
    /// Error serializing data
    SerializeError(String),
    /// Error deserializing data
    DeserializeError(String),
    /// Error setting data
    #[cfg(target_arch = "wasm32")]
    SetError(String),
    /// No data found for key
    NotFound(String),
    /// Database error (SQLite)
    #[cfg(not(target_arch = "wasm32"))]
    DbError(String),
}

/// Storage trait defining common operations
pub trait StorageProvider {
    /// Save data to storage
    fn save<T: Serialize>(&self, key: &str, data: &T) -> Result<(), StorageError>;

    /// Load data from storage
    fn load<T: DeserializeOwned>(&self, key: &str) -> Result<T, StorageError>;
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use web_sys::Storage;

    pub struct WebStorage;

    impl WebStorage {
        pub fn new() -> Self {
            Self {}
        }

        /// Gets the localStorage object.
        fn local_storage(&self) -> Result<Storage, StorageError> {
            let window = web_sys::window().ok_or(StorageError::AccessError)?;
            window
                .local_storage()
                .map_err(|_| StorageError::AccessError)?
                .ok_or(StorageError::AccessError)
        }
    }

    impl StorageProvider for WebStorage {
        fn save<T: Serialize>(&self, key: &str, data: &T) -> Result<(), StorageError> {
            let storage = self.local_storage()?;
            let json = serde_json::to_string(data)
                .map_err(|e| StorageError::SerializeError(e.to_string()))?;

            storage.set_item(key, &json).map_err(|e| {
                StorageError::SetError(format!("Failed to set item for key {}: {:?}", key, e))
            })
        }

        fn load<T: DeserializeOwned>(&self, key: &str) -> Result<T, StorageError> {
            let storage = self.local_storage()?;
            let json = storage
                .get_item(key)
                .map_err(|_| StorageError::AccessError)?
                .ok_or_else(|| StorageError::NotFound(format!("No data found for key: {}", key)))?;

            serde_json::from_str(&json).map_err(|e| {
                StorageError::DeserializeError(format!(
                    "Failed to deserialize data for key {}: {}",
                    key, e
                ))
            })
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod desktop {
    use super::*;

    #[cfg(feature = "desktop")]
    use {
        rusqlite::{Connection, params},
        std::path::PathBuf,
    };

    pub struct SqliteStorage {
        #[cfg(feature = "desktop")]
        conn: Connection,
    }

    impl SqliteStorage {
        #[cfg(feature = "desktop")]
        pub fn new() -> Result<Self, StorageError> {
            let app_dir = dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("editor");

            info!("App directory: {:?}", app_dir);

            std::fs::create_dir_all(&app_dir).map_err(|e| {
                let error_msg = format!("Failed to create app directory: {}", e);
                error!("{}", error_msg);
                StorageError::DbError(error_msg)
            })?;

            let db_path = app_dir.join("storage.db");
            let conn = Connection::open(&db_path).map_err(|e| {
                let error_msg = format!("Failed to open database at {:?}: {}", db_path, e);
                error!("{}", error_msg);
                StorageError::DbError(error_msg)
            })?;

            // Create table if it doesn't exist
            conn.execute(
                "CREATE TABLE IF NOT EXISTS kv_store (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                )",
                [],
            )
            .map_err(|e| {
                let error_msg = format!("Failed to create table: {}", e);
                error!("{}", error_msg);
                StorageError::DbError(error_msg)
            })?;

            Ok(Self { conn })
        }

        #[cfg(not(feature = "desktop"))]
        pub fn new() -> Result<Self, StorageError> {
            Err(StorageError::DbError(
                "Desktop feature not enabled".to_string(),
            ))
        }
    }

    #[cfg(feature = "desktop")]
    impl StorageProvider for SqliteStorage {
        fn save<T: Serialize>(&self, key: &str, data: &T) -> Result<(), StorageError> {
            let json = serde_json::to_string(data).map_err(|e| {
                StorageError::SerializeError(format!(
                    "Failed to serialize data for key {}: {}",
                    key, e
                ))
            })?;

            self.conn
                .execute(
                    "INSERT OR REPLACE INTO kv_store (key, value) VALUES (?1, ?2)",
                    params![key, json],
                )
                .map_err(|e| {
                    let error_msg = format!("Failed to save data for key {}: {}", key, e);
                    error!("{}", error_msg);
                    StorageError::DbError(error_msg)
                })?;

            info!("Data saved successfully for key: {}", key);
            Ok(())
        }

        fn load<T: DeserializeOwned>(&self, key: &str) -> Result<T, StorageError> {
            let mut stmt = self
                .conn
                .prepare("SELECT value FROM kv_store WHERE key = ?1")
                .map_err(|e| {
                    let error_msg = format!("Failed to prepare query for key {}: {}", key, e);
                    error!("{}", error_msg);
                    StorageError::DbError(error_msg)
                })?;

            let json: String = stmt
                .query_row(params![key], |row| row.get(0))
                .map_err(|e| {
                    if let rusqlite::Error::QueryReturnedNoRows = e {
                        debug!("No data found for key: {}", key);
                        StorageError::NotFound(format!("No data found for key: {}", key))
                    } else {
                        let error_msg = format!("Failed to query data for key {}: {}", key, e);
                        error!("{}", error_msg);
                        StorageError::DbError(error_msg)
                    }
                })?;

            serde_json::from_str(&json).map_err(|e| {
                let error_msg = format!("Failed to deserialize data for key {}: {}", key, e);
                error!("{}", error_msg);
                StorageError::DeserializeError(error_msg)
            })
        }
    }

    #[cfg(not(feature = "desktop"))]
    impl StorageProvider for SqliteStorage {
        fn save<T: Serialize>(&self, _key: &str, _data: &T) -> Result<(), StorageError> {
            Err(StorageError::DbError(
                "Desktop feature not enabled".to_string(),
            ))
        }

        fn load<T: DeserializeOwned>(&self, _key: &str) -> Result<T, StorageError> {
            Err(StorageError::DbError(
                "Desktop feature not enabled".to_string(),
            ))
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use web::WebStorage as Storage;

/// Get the platform-specific storage provider
pub fn get_storage() -> Result<impl StorageProvider, StorageError> {
    #[cfg(target_arch = "wasm32")]
    {
        Ok(web::WebStorage::new())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        desktop::SqliteStorage::new()
    }
}

/// Saves data to storage.
///
/// # Arguments
/// * `key` - The key under which to store the data
/// * `data` - The data to store, must implement Serialize
///
/// # Returns
/// * `Ok(())` if the data was stored successfully
/// * `Err(StorageError)` if there was an error storing the data
///
/// # Example
/// ```
/// # use todo::models::TodoList;
/// # use todo::utils::constants::storage::TODO_STORAGE_KEY;
/// # use todo::utils::save;
/// let todo_list = TodoList::new();
/// let result = save(TODO_STORAGE_KEY, &todo_list);
/// ```
pub fn save<T: Serialize>(key: &str, data: &T) -> Result<(), StorageError> {
    let storage = get_storage().map_err(|e| {
        #[cfg(feature = "desktop")]
        error!("Failed to get storage provider: {:?}", e);
        e
    })?;

    storage.save(key, data).map_err(|e| {
        #[cfg(feature = "desktop")]
        error!("Failed to save data for key {}: {:?}", key, e);
        e
    })
}

/// Loads data from storage.
///
/// # Arguments
/// * `key` - The key under which the data is stored
///
/// # Returns
/// * `Ok(T)` containing the loaded data
/// * `Err(StorageError)` if there was an error loading the data
///
/// # Example
/// ```
/// # use todo::models::TodoList;
/// # use todo::utils::constants::storage::TODO_STORAGE_KEY;
/// # use todo::utils::load;
/// let result: Result<TodoList, _> = load(TODO_STORAGE_KEY);
/// match result {
///     Ok(todo_list) => println!("Loaded {} todos", todo_list.total_count()),
///     Err(e) => println!("Error loading todos: {:?}", e),
/// }
/// ```
pub fn load<T: DeserializeOwned>(key: &str) -> Result<T, StorageError> {
    let storage = get_storage().map_err(|e| {
        #[cfg(feature = "desktop")]
        error!("Failed to get storage provider: {:?}", e);
        e
    })?;

    storage.load(key).map_err(|e| {
        if let StorageError::NotFound(_) = &e {
            #[cfg(feature = "desktop")]
            debug!("No data found for key: {}", key);
        } else {
            #[cfg(feature = "desktop")]
            error!("Failed to load data for key {}: {:?}", key, e);
        }
        e
    })
}
