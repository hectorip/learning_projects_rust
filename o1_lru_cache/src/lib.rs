// Necesitamos una estructura de datos que nos permita
// mantener el orden de los elementos e intercambiarlos con
// un costo de O(n)
//
// Una lista doblemente ligada cumple con ese objetivo
//
//
use std::{cell::RefCell, rc::Rc}; // Necesitamos Rc para poder tener referencias a los nodos

struct Node {
    value: i32,
    key: i32,
    prev: Option<Rc<RefCell<Self>>>,
    next: Option<Rc<RefCell<Self>>>,
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
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl DoubleLinkedList {
    fn new() -> DoubleLinkedList {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }
    fn push_front(&mut self, key: i32, value: i32) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node::new(key, value)));
        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(head);
            self.head = Some(Rc::clone(&node));
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node)); // Maybe we can avoid the clone
        }
        node
    }
    fn move_front(&mut self, node: Rc<RefCell<Node>>) {
        // Recibe una referencia un nodo y lo mueve al frente de la lista

        // Si el nodo ya está al frente no hacemos nada
        if Rc::ptr_eq(&self.head.as_ref().unwrap(), &node) {
            return;
        }

        if let Some(prev) = node.borrow_mut().prev.take() {
            prev.borrow_mut().next = node.borrow_mut().next.take(); // No iporta si es None
                                                                    // Si el nodo es la cola, tenemos apuntar la cola al nodo previo.
            if Rc::ptr_eq(&self.tail.as_ref().unwrap(), &node) {
                self.tail = Some(Rc::clone(&prev));
            }
            // Si el nodo no es el último, tenemos que actualizar su referencia
            if let Some(next) = node.borrow_mut().next.take() {
                next.borrow_mut().prev = Some(Rc::clone(&prev));
            }

        }
        let next = node.borrow_mut().next.as_ref();


        // En otro caso, lo ponermos como el nuevo head
    }
    fn pop_tail(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(tail) = self.tail {
            self.tail = tail.borrow().prev;
            self.tail.unwrap().borrow_mut().next = None;
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
    cache: std::collections::HashMap<i32, Rc<RefCell<Node>>>,
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
        if let Some(node) = self.cache.get(&key) {
            self.queue.move_front(*node);
            node.borrow().value
        } else {
            -1
        }
    }
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            node.borrow().value = value;
            self.queue.move_front(*node)
        } else {
            if self._current_capacity >= self.capacity {
                self._current_capacity -= 1;
                let node = self.queue.pop_tail().unwrap();
                self.cache.remove(&node.borrow().key);
            }
            self._current_capacity += 1;
            let node = self.queue.push_front(key, value);
            self.cache.insert(key, node);
        }
    }
}
