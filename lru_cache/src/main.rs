// Implementing LRU Cache in Rust
// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

// Implement the LRUCache class:

// LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
// int get(int key) Return the value of the key if the key exists, otherwise return -1.
// void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
// The functions get and put must each run in O(1) average time complexity.
use std::collections::HashMap;
use std::collections::VecDeque; // Dequeue
                                // 1 -> 2 -> 3
                                // -> 1 -> 2 -> 3

fn main() {
    println!("LRU Cache Implementation");
    let mut cache = LRUCache::new(2);
    println!("Cache: {:?}", cache);
    cache.put(1, 5);
    println!("Cache: {:?}", cache);
    cache.put(2, 10);
    cache.put(3, 15);
    println!("Get 1: {}", cache.get(1));
    println!("Get 2: {}", cache.get(2));
    println!("Get 3: {}", cache.get(3));
}

#[derive(Debug)]
struct LRUCache {
    capacity: usize, // The number of elements the cache can hold
    cache: HashMap<i32, i32>, // The key and the index in the queue
    queue: VecDeque<i32>, // We need to remove elemenents from both sides and a double linked list
    _current_size: usize,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            queue: VecDeque::with_capacity(capacity),
            _current_size: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let result = self.cache.get(&key);
        match result {
            Some(value) => {
                self.queue.retain(|&x| x != key); // Remove the key from the queue
                self.queue.insert(self.capacity - 1, key);
                *value
            }
            None => -1,
        }
    }

    fn _update_queue(&mut self, key: i32) {
        self.queue.retain(|&x| x != key); // Remove the key from the queue
        self.queue.insert(self.capacity - 1, key);
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.cache.insert(key, value);
            self._update_queue(key);
        } else {
            if self._current_size == self.capacity {
                let key_to_remove: i32 = self.queue.remove(0).expect("Error fatal");
                self.cache.remove(&key_to_remove);
                self._current_size -= 1;
            }
            self._current_size += 1;
            self.cache.insert(key, value);
            self.queue.push_back(key);
        }
    }
}
