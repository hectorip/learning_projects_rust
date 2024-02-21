fn main() {
    println!("Hello, world!");
    let mut cache = o1_lru_cache::LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("{}", cache.get(1)); // 1
    cache.put(3, 3);
    println!("{}", cache.get(2)); // -1
    cache.put(4, 4);
    println!("{}", cache.get(1)); // -1
    println!("{}", cache.get(3)); // 3
    println!("{}", cache.get(4)); // 4

    //for i in 1..=3 {
    //   println!("Cache de {} = {}", i, cache.get(i))
    //}
}
