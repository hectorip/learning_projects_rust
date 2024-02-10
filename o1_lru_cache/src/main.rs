fn main() {
    println!("Hello, world!");
    let mut cache = o1_lru_cache::LRUCache::new(4);
    cache.put(1, 2);
    cache.put(2, 4);
    cache.put(3, 6);

    for i in 1..=3 {
        println!("Cache de {} = {}", i, cache.get(i))
    }
}
