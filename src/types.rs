use std::collections::HashMap;
use std::hash::Hash;
use std::collections::VecDeque;
use std::time::SystemTime;


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
pub struct CacheSystem<K: Eq + Hash + Clone, T> {
    entries: HashMap<K, CacheEntry<T>>,
    order: VecDeque<K>,
    recently_used: VecDeque<K>,
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
        self.entries.insert(key, CacheEntry::new(data));

        Ok(())
    }

    pub fn get_data(&mut self, key: K) -> Option<&CacheEntry<T>>{
        if let Some(entry) = self.entries.get(&key) {
            self.recently_used.push_back(key);
            Some(entry)
        } else {
            None
        }
    }
}

impl <K: Eq + Hash + Clone, T> CachePolicy<K> for CacheSystem<K, T> {
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
