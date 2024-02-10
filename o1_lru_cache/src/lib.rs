// Necesitamos una estructura de datos que nos permita
// mantener el orden de los elementos e intercambiarlos con
// un costo de O(n)
//
// Una lista doblemente ligada cumple con ese objetivo
//

struct Node<'a> {
    value: i32,
    prev: Option<&'a mut Node<'a>>,
    next: Option<&'a mut Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(value: i32) -> Node<'a> {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
}

pub struct LRUCache<'a> {
    capacity: usize,
    cache: std::collections::HashMap<i32, Node<'a>>,
    queue: Option<&'a mut Node<'a>>,
    _current_capacity: usize,
}

impl<'a> LRUCache<'a> {
    pub fn new(capacity: i32) -> LRUCache<'a> {
        LRUCache {
            capacity: capacity as usize,
            cache: std::collections::HashMap::new(),
            queue: None,
            _current_capacity: 0,
        }
    }
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key) {
            node.value
        } else {
            -1
        }
    }
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get_mut(&key) {
            node.value = value;
        } else {
            if self._current_capacity >= self.capacity {
                self._current_capacity -= 1;
            }
            self._current_capacity += 1;
            let mut node = Node::new(value);
            if let Some(queue) = self.queue {
                node.next = Some(queue)
            }
            self.cache.insert(key, node);
        }
        let node = Node::new(value);
    }
}
