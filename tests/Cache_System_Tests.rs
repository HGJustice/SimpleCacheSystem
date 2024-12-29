use std::time::{SystemTime, UNIX_EPOCH};

use example::types::CacheSystem;

#[test]
fn test_new_cache_builds_is_empty(){
    let cache = CacheSystem::<u32, String>::new();
    let result = cache.get_data(1);
    assert!(result.is_none());
}

#[test]
fn test_insert_and_get(){
    let mut cache = CacheSystem::<u32, String>::new();
    let test_time = SystemTime::now();

    cache.insert_data(1, "wag 1".to_string());
    let result = cache.get_data(1);
    assert_eq!(result.unwrap().value, "wag 1");
    assert!(result.unwrap().timestamp > test_time);
}
