// Necesitamos una estructura de datos que nos permita
// mantener el orden de los elementos e intercambiarlos con
// un costo de O(n)
//
// Una lista doblemente ligada cumple con ese objetivo
//
//
use std::{cell::RefCell, rc::Rc}; // Necesitamos Rc para poder tener referencias a los nodos

#[derive(Debug)]
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
    fn print(&self) {
        let mut current = self.head.to_owned();
        let mut cadena = String::new();
        while let Some(node) = current {
            cadena = format!("{cadena} -> {}", node.borrow().key);
            current = node.borrow().next.to_owned();
        }
        println!("{}", cadena);
    }
    fn push_front(&mut self, key: i32, value: i32) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node::new(key, value)));
        println!("{:?}", node);
        if let Some(head) = self.head.take() {
            println!("Inserting {} at the front", key);
            node.borrow_mut().next = Some(head.clone());
            head.borrow_mut().prev = Some(Rc::clone(&node));
            self.head = Some(Rc::clone(&node));
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node)); // Maybe we can avoid the clone
        }
        println!("head: {}", self.head.as_ref().unwrap().borrow().key);
        println!("tail: {}", self.tail.as_ref().unwrap().borrow().key);
        node
    }
    fn move_front(&mut self, node: Rc<RefCell<Node>>) {
        // Recibe una referencia un nodo y lo mueve al frente de la lista

        // Si el nodo ya está al frente no hacemos nada
        if Rc::ptr_eq(&self.head.as_ref().unwrap(), &node) {
            return;
        }
        let mut borrowed_node = node.borrow_mut();
        if let Some(prev) = borrowed_node.prev.take() {
            prev.borrow_mut().next = borrowed_node.next.clone(); // No iporta si es None
                                                                 // Si el nodo es la cola, tenemos apuntar la cola al nodo previo.
            if Rc::ptr_eq(&self.tail.as_ref().unwrap(), &node) {
                self.tail = Some(Rc::clone(&prev));
            }
            // Si el nodo no es el último, tenemos que actualizar su referencia
            if let Some(next) = borrowed_node.next.take() {
                next.borrow_mut().prev = Some(Rc::clone(&prev));
            }
        }
        // Actualizamos las referencias del nodo
        // El nodo que estaba al frente ahora es el siguiente
        if let Some(head) = &self.head {
            head.borrow_mut().prev = Some(Rc::clone(&node));
        }

        let head = self.head.take();
        borrowed_node.next = head;
        borrowed_node.prev = None;
        self.head = Some(Rc::clone(&node));
        println!("Lista actualizada");
    }

    fn pop_tail(&mut self) -> Option<i32> {
        self.tail.take().map(|tail| {
            match tail.borrow_mut().prev.take() {
                Some(prev) => {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                }
                None => {
                    println!("No more elements");
                    self.head.take();
                }
            }
            tail.borrow().key
        })
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
            self.queue.move_front(node.clone());

            self.queue.print();
            node.borrow().value
        } else {
            -1
        }
    }
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            node.borrow_mut().value = value;
            self.queue.move_front(node.clone())
        } else {
            if self._current_capacity >= self.capacity {
                self._current_capacity -= 1;
                let key = self.queue.pop_tail().unwrap();
                self.cache.remove(&key);
                println!("Removing {}", key);
            }
            self._current_capacity += 1;
            let node = self.queue.push_front(key, value);
            self.cache.insert(key, node.clone());
            println!("Current State");
            self.queue.print();
        }
    }
}
