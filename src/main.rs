mod types;
mod errors;
use crate::types::CacheSystem;
use crate::types::CacheEntry;

fn main() {

    let cache = CacheSystem::<u32, String>::new();
    let data = CacheEntry::<String>::new("hey".to_string());
  
}
