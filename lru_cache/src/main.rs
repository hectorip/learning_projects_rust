// Implementing LRU Cache in Rust
// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

// Implement the LRUCache class:

// LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
// int get(int key) Return the value of the key if the key exists, otherwise return -1.
// void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
// The functions get and put must each run in O(1) average time complexity.


fn main() {
    println!("LRU Cache Implementation");
}
struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    queue: Vec<i32>
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            queue: Vec::new()
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.cache.get(&key) {
            Some(value) => {
                self.queue.retain(|&x| x != key);
                self.queue.push(key);
                *value
            },
            None => -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.cache.insert(key, value);
            self.queue.retain(|&x| x != key);
            self.queue.push(key);
        } else {
            if self.cache.len() == self.capacity {
                let key_to_remove = self.queue.remove(0);
                self.cache.remove(&key_to_remove);
            }
            self.cache.insert(key, value);
            self.queue.push(key);
        }
    }
}

