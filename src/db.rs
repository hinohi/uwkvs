use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{Key, Value};

#[derive(Debug, Clone)]
pub struct Db {
    map: Arc<RwLock<HashMap<Key, Value>>>,
}

impl Db {
    pub fn new(capacity: usize) -> Db {
        Db {
            map: Arc::new(RwLock::new(HashMap::with_capacity(capacity))),
        }
    }

    pub async fn get(&self, key: &Key) -> Option<Value> {
        self.map.read().await.get(&key).copied()
    }

    pub async fn set(&self, key: Key, value: Value) -> Option<Value> {
        self.map.write().await.insert(key, value)
    }
}
