

use std::collections::HashMap;
use std::hash::Hash;

use crate::errors::CacheDataError;
use crate::errors::CacheSystemError;

const MAX_SIZE: u32 = 5;

pub struct CacheSystem<K: Eq + Hash, T> {
    entries: HashMap<K, CacheEntry<T>>,
}

impl<K: Eq + Hash, T> CacheSystem<K, T> {
    pub fn new() -> CacheSystem<K, T> {
        CacheSystem {
            entries: HashMap::new(),
        }
    }

    pub fn insert_data(&mut self,key: K,  data: T) -> Result<(), CacheSystemError>{
        if self.entries.len() >= MAX_SIZE as usize {
           return Err(CacheSystemError::CacheFull);
        }
        self.entries.insert(key, CacheEntry(data));

        Ok(())
    }
}

pub struct CacheEntry<T> {
    value: T,
}

impl <T> CacheEntry<T> {
    pub fn new(value: T, ) -> CacheEntry<T> {
        CacheEntry {
            value
        }
    }
    
}
