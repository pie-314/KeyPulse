use crate::models::ApiKey;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::sync::{Arc, RwLock};

const KEYS_FILE: &str = "keys.json";

#[derive(Clone, Serialize, Deserialize)]
pub struct AppState {
    #[serde(skip)]
    pub keys: Arc<DashMap<String, ApiKey>>,
    #[serde(skip)]
    pub total_requests_this_minute: Arc<RwLock<u32>>,
}

impl AppState {
    pub fn new() -> Self {
        let keys = Arc::new(DashMap::new());
        let total_requests_this_minute = Arc::new(RwLock::new(0));
        Self {
            keys,
            total_requests_this_minute,
        }
    }

    pub fn load() -> Self {
        let keys = match fs::read_to_string(KEYS_FILE) {
            Ok(data) => {
                let keys: Vec<ApiKey> = serde_json::from_str(&data).unwrap_or_default();
                keys.into_iter().map(|k| (k.key.clone(), k)).collect()
            }
            Err(_) => DashMap::new(),
        };
        let total_requests_this_minute = Arc::new(RwLock::new(0));
        Self {
            keys: Arc::new(keys),
            total_requests_this_minute,
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let keys: Vec<ApiKey> = self.keys.iter().map(|entry| entry.value().clone()).collect();
        let data = serde_json::to_string_pretty(&keys)?;
        let mut file = fs::File::create(KEYS_FILE)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
