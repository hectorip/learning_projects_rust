// Necesitamos una estructura de datos que nos permita
// mantener el orden de los elementos e intercambiarlos con
// un costo de O(n)
//
// Una lista doblemente ligada cumple con ese objetivo
//
//
use std::rc::Rc; // Necesitamos Rc para poder tener referencias a los nodos

struct Node {
    value: i32,
    key: i32,
    prev: Option<Rc<Self>>,
    next: Option<Rc<Self>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Node {
        Node {
            value,
            key,
            prev: None,
            next: None,
        }
    }
}

struct DoubleLinkedList {
    head: Option<Rc<Node>>,
    tail: Option<Rc<Node>>,
}

impl DoubleLinkedList {
    fn new() -> DoubleLinkedList {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }
    fn push_front(&mut self, key: i32, value: i32) -> Rc<Node> {
        let mut node = Rc::new(Node::new(key, value));
        match self.head {
            Some(_) => {
                node.next = Some(*self.head.as_ref().unwrap());
                self.head.as_mut().unwrap().prev = Some(node);
            }
            None => {
                self.tail = Some(node);
            }
        }
        self.head = Some(node);
        node
    }
    fn move_front(&mut self, node: &mut Rc<Node>) {
        // Mueve un nodo EXISTENTE al frente de la lista
        if let Some(mut prev) = node.prev {
            prev.next = node.next;
        }
        node.next = self.head;
        self.head = Some(*node);
    }
    fn pop_tail(&self) -> Option<Rc<Node>> {
        if let Some(tail) = self.tail {
            self.tail = tail.prev;
            self.tail.unwrap().next = None;
            Some(tail)
        } else {
            None
        }
    }
    // No necesitamos implementar más funciones porque no las usamos, los nodos
    // se mueven hacia atrás cuando otros nodos son insertados
}

pub struct LRUCache {
    capacity: usize,
    cache: std::collections::HashMap<i32, Rc<Node>>,
    queue: DoubleLinkedList,
    _current_capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> LRUCache {
        LRUCache {
            capacity: capacity as usize,
            cache: std::collections::HashMap::new(),
            queue: DoubleLinkedList::new(),
            _current_capacity: 0,
        }
    }
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get_mut(&key) {
            self.queue.move_front(node);
            node.value
        } else {
            -1
        }
    }
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get_mut(&key) {
            node.value = value;
            self.queue.move_front(node)
        } else {
            if self._current_capacity >= self.capacity {
                self._current_capacity -= 1;
                let node = self.queue.pop_tail().unwrap();
                self.cache.remove(&node.key);
            }
            self._current_capacity += 1;
            let node = self.queue.push_front(key, value);
            self.cache.insert(key, node);
        }
    }
}
