pub mod types;
pub mod errors;
use crate::types::CacheSystem;
use crate::types::CacheEntry;

fn main() {

    let mut cache = CacheSystem::<u32, String>::new();
    cache.insert_data(1, "wag1".to_string());
    
    let data = cache.get_data(1);
    println!("{:?}", data);
    
  
}
