use example::types::{CacheSystem, CachePolicy, CacheEntry};
use example::errors::CacheSystemError;
use std::collections::HashMap;

 mod tests {
 
    use example::types::Serializer;

    use super::*;

    #[test]
    fn test_basic_cache_system(){
        let mut cache = CacheSystem::<u32, String>::new();

        assert!(cache.entries.is_empty());
        assert!(cache.order.is_empty());
        assert!(cache.recently_used.is_empty());

        cache.insert_data(1, String::from("Hello")).unwrap();
        let result = cache.get_data(1).unwrap();
        assert_eq!(result.value, String::from("Hello"));

        cache.insert_data(2, String::from("hi")).unwrap();
        cache.insert_data(3, String::from("lol")).unwrap(); 
        assert!(matches!(cache.insert_data(4, String::from("should fail")), Err(CacheSystemError::CacheFull))); // 3 Max size test
       
        cache.clear_cache().unwrap();
        let empty = cache.get_data(1);
        assert!(empty.is_none());
    }

    #[test]
    fn test_fifo_lru_traits(){
        let mut fifo_cache = CacheSystem::<u32, String>::new();
        fifo_cache.insert_data(1, String::from("hi")).unwrap();
        fifo_cache.insert_data(2, String::from("hello")).unwrap();
        fifo_cache.insert_data(3, String::from("last one")).unwrap();
        fifo_cache.fifo().unwrap();
        let empty = fifo_cache.get_data(1);
        assert!(empty.is_none());
        assert!(matches!(fifo_cache.fifo(), Err(CacheSystemError::CacheNotFull)));

        let mut lru_cache = CacheSystem::<u32, String>::new();
        lru_cache.insert_data(1, String::from("least used")).unwrap();
        lru_cache.insert_data(2, String::from("hey")).unwrap();
        lru_cache.insert_data(3, String::from("bruh")).unwrap();

        let _ = lru_cache.get_data(2);
        let _ = lru_cache.get_data(3);

        lru_cache.lru().unwrap();
        let empty = lru_cache.get_data(1);
        assert!(empty.is_none());
        assert!(matches!(lru_cache.lru(), Err(CacheSystemError::CacheNotFull)));
    }

    #[test]
    fn test_serialize_deserialize(){
        let mut cache = CacheSystem::<u32, String>::new();
        cache.insert_data(1, String::from("Hello")).unwrap();
        
        let json = cache.serialize_json().unwrap();
        let deserialized_json: HashMap<u32, CacheEntry<String>> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized_json.get(&1).unwrap().value, "Hello".to_string());
        
        let binary_result = cache.serialize_binary().unwrap();
        let deserialized_binary: HashMap<u32, CacheEntry<String>> = bincode::deserialize(&binary_result).unwrap();
        assert_eq!(deserialized_binary.get(&1).unwrap().value, "Hello".to_string());
    }
}