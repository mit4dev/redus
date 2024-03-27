use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Duration, Utc};

use crate::command::{SetExpiration, SetOptions};

#[derive(Debug, Clone)]
pub struct Data {
    pub val: String,

    /// Timestamp before
    pub expires_at: Option<u64>,
}

pub type Store = Arc<Mutex<HashMap<String, Data>>>;

#[derive(Debug, Clone)]
pub struct StoreService {
    store: Store,
}

impl StoreService {
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    pub fn get(&self, key: String) -> anyhow::Result<Option<String>> {
        let store = self.store.lock().unwrap();

        let res = store.get(&key).and_then(|data| {
            data.expires_at
                .map_or(Some(data.clone().val), |expires_at| {
                    let expires =
                        DateTime::from_timestamp_millis(i64::try_from(expires_at).unwrap())
                            .unwrap();

                    expires
                        .timestamp_millis()
                        .ge(&Utc::now().timestamp_millis())
                        .then(|| data.val.clone())
                })
        });

        Ok(res)
    }

    pub fn set(&self, key: String, val: String) -> anyhow::Result<()> {
        let mut store = self.store.lock().unwrap();

        store.insert(
            key,
            Data {
                val,
                expires_at: None,
            },
        );

        Ok(())
    }

    pub fn set_exp(
        &self,
        key: String,
        val: String,
        ttl: String,
        _exp_type: SetExpiration,
        _options: Option<SetOptions>,
    ) -> anyhow::Result<()> {
        let mut store = self.store.lock().unwrap();

        // TODO: calculate exp type!
        let expires_at =
            Utc::now() + Duration::try_milliseconds(ttl.parse::<i64>().unwrap()).unwrap();

        store.insert(
            key,
            Data {
                val,
                expires_at: Some(expires_at.timestamp_millis() as u64),
            },
        );

        Ok(())
    }
}
