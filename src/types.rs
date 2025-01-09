use std::collections::HashMap;
use std::hash::Hash;
use std::collections::VecDeque;

use serde::Deserialize;
use serde::Serialize;

use crate::errors::CacheSystemError;
use crate::errors::SerializeError;

const MAX_CACHE_SIZE: u32 = 3;

pub trait CachePolicy<K> {
    fn lru(&mut self) -> Result<(), CacheSystemError>;
    fn fifo(&mut self) -> Result<(), CacheSystemError>;
}

pub trait Serializer<T> {
    fn serialize_json(&self) -> Result<String, SerializeError>;
    fn serialize_binary(&self) -> Result<Vec<u8>, SerializeError>;
    fn deserialize(&self, data: &str) -> Result<T, SerializeError>;
}

#[derive(Debug)]
pub struct CacheSystem<K: Eq + Hash + Clone, T> {
    pub entries: HashMap<K, CacheEntry<T>>,
    pub order: VecDeque<K>,
    pub recently_used: VecDeque<K>,
}

impl<K: Eq + Hash + Clone, T> CacheSystem<K, T> {
    pub fn new() -> CacheSystem<K, T> {
        CacheSystem {
            entries: HashMap::new(),
            order: VecDeque::new(),
            recently_used: VecDeque::new(),
        }
    }

    pub fn insert_data(&mut self, key: K, data: T) -> Result<(), CacheSystemError> {
        if self.entries.len() >= MAX_CACHE_SIZE as usize {
           return Err(CacheSystemError::CacheFull);
        }
        self.order.push_back(key.clone());
        self.recently_used.push_back(key.clone());
        self.entries.insert(key, CacheEntry::new(data));

        Ok(())
    }

    pub fn get_data(&mut self, key: K) -> Option<&CacheEntry<T>>{
        if let Some(entry) = self.entries.get(&key) {
            self.recently_used.retain(|k| k != &key);
            self.recently_used.push_back(key);
            Some(entry)
        } else {
            None
        }
    }

    pub fn clear_cache(&mut self) -> Result<(), CacheSystemError> {
        if self.entries.len() == 0 as usize {
            return Err(CacheSystemError::CacheAlreadyEmpty)
        }
        self.entries.clear();
        self.order.clear();
        self.recently_used.clear();
        Ok(())
    }
}

impl <K: Eq + Hash + Clone, T> CachePolicy<K> for CacheSystem<K, T> { // because the typee is generic we need to add these traits?
    fn fifo(&mut self) -> Result<(), CacheSystemError> {
        if self.entries.len() < MAX_CACHE_SIZE as usize {
            return Err(CacheSystemError::CacheNotFull);
        }

        if let Some(oldest_key) = self.order.pop_front(){
            self.entries.remove(&oldest_key);
            return Ok(())
        } else {
            Err(CacheSystemError::InvalidKey)
        }
    }

    fn lru(&mut self) -> Result<(), CacheSystemError> {
        if self.entries.len() < MAX_CACHE_SIZE as usize {
            return Err(CacheSystemError::CacheNotFull);
        }

        if let Some(oldest_key) = self.recently_used.pop_front(){
            self.entries.remove(&oldest_key);
            return Ok(())
        }else {
            Err(CacheSystemError::InvalidKey)
        }
    }
        
}

impl <K: Serialize + Eq + Hash + Clone, T: Serialize + for<'a> Deserialize<'a>> Serializer<T> for CacheSystem<K, T> {
    fn serialize_json(&self) -> Result<String, SerializeError> {
        serde_json::to_string(&self.entries).map_err(|_| SerializeError::JsonError)
    }
    fn serialize_binary(&self) -> Result<Vec<u8>, SerializeError> {
        bincode::serialize(&self.entries).map_err(|_| SerializeError::BinaryError)
    }
    fn deserialize(&self, data: &str) -> Result<T, SerializeError> {
        serde_json::from_str(data).map_err(|_| SerializeError::DeserializeError)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CacheEntry<T> {
    pub value: T,
}

impl <T> CacheEntry<T> {
    pub fn new(value: T, ) -> CacheEntry<T> {
        CacheEntry {
            value,
        }
    }
}