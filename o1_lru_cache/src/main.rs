fn main() {
    println!("Hello, world!");
}

struct LRUCache {
    capacity: usize, // The number of elements the cache can hold
    cache: HashMap<i32, LRUCacheNode>,
    queue: VecDeque<L>, // We need to remove elemenents from both sides
    _current_size: usize,
}

struct LRUCacheNode {
    value: i32,
    next: Option<&LRUCacheNode>,
    prev: Option<&LRUCacheNode>,
}
