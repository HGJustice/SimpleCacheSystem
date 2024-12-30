use std::collections::HashMap;
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::CacheDataError;
use crate::errors::CacheSystemError;
use crate::errors::SerializeError;

const MAX_CACHE_SIZE: u32 = 10;

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
        if self.entries.len() >= MAX_CACHE_SIZE as usize {
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

impl <K: Eq + Hash, T> CachePolicy<K> for CacheSystem<K, T> {
    fn fifo(&mut self) -> Result<(), CacheSystemError> {
        if self.entries.len() < MAX_CACHE_SIZE as usize {
            return Err(CacheSystemError::CacheNotFull);
        }
          let mut iter = self.entries.iter();
          let mut oldest;
        
          if let Some((mut oldest_key, mut oldest_value)) = iter.next() {
            for (key, value) in iter {
                if value.creation_timestamp < oldest_value.creation_timestamp {
                    oldest_key = key;
                    oldest_value = value;
                }
               oldest = oldest_key;
            }
            self.entries.remove(&oldest);
            println!("Oldest data removed");
            return Ok(());
          } 
          Err(CacheSystemError::InvalidValue)
        }
        
}


// impl Serializer<T> for CacheSystem<K, T> {
//     fn serialize_json(&self) -> Result<String, SerializeError> {
        
//     }

// }

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct CacheEntry<T> {
    pub value: T,
    pub creation_timestamp: SystemTime,
    pub last_accessed_timestamp: SystemTime,
   
}

impl <T> CacheEntry<T> {
    pub fn new(value: T, ) -> CacheEntry<T> {
        CacheEntry {
            value,
            creation_timestamp: SystemTime::now(),
            last_accessed_timestamp: SystemTime::now(),
        }
    }
}
