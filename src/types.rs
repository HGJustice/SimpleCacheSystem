use std::collections::HashMap;
use std::hash::Hash;

use crate::errors::CacheDataError;
use crate::errors::CacheSystemError;
use crate::errors::SerializeError;

const MAX_SIZE: u32 = 5;

pub trait CachePolicy<K> {
    fn lru(&mut self) -> Option<K>;
    fn fifo(&mut self) -> Option<K>;
}

pub trait Serializer<T> {
    fn serialize_json(&self) -> Result<String, SerializeError>;
    fn serialize_binary(&self) -> Result<Vec<u8>, SerializeError>;
    fn deserialize(&self, data: &str) -> Result<T, SerializeError>;
}

#[derive(Debug)]
pub struct CacheSystem<K: Eq + Hash, T> {
    entries: HashMap<K, CacheEntry<T>>,
}

impl<K: Eq + Hash, T> CacheSystem<K, T> {
    pub fn new() -> CacheSystem<K, T> {
        CacheSystem {
            entries: HashMap::new(),
        }
    }

    pub fn insert_data(&mut self, key: K, data: T) -> Result<(), CacheSystemError> {
        if self.entries.len() >= MAX_SIZE as usize {
           return Err(CacheSystemError::CacheFull);
        }
        self.entries.insert(key, CacheEntry::new(data));

        Ok(())
    }

    pub fn get_data(&self, key: K) -> Option<&CacheEntry<T>>{
        if self.entries.is_empty() {
            return None;
        }
        self.entries.get(&key)
    }
}

impl <K: Eq + Hash, T> CachePolicy<K> for CacheSystem<K, T>{
    fn fifo(&mut self) -> Option<K> {
        
    }
    fn lru(&mut self) -> Option<K> {
        
    }
} 

#[derive(Debug)]
pub struct CacheEntry<T> {
    pub value: T,
}

impl <T> CacheEntry<T> {
    pub fn new(value: T, ) -> CacheEntry<T> {
        CacheEntry {
            value
        }
    }
}
