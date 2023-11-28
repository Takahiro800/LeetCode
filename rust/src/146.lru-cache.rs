use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;

// 途中までしかできていない


#[derive(Debug)]
struct ListNode {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct DoubleListNode {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl DoubleListNode {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn get_head(&self) -> Option<Rc<RefCell<ListNode>>> {
        match &self.head {
            Some(head) => Some(head.clone()),
            None => None,
        }
    }

    fn get_tail(&self) -> Option<Rc<RefCell<ListNode>>> {
        match &self.tail {
            Some(tail) => Some(tail.clone()),
            None => None,
        }
    }

    pub fn add_front(&mut self, key: i32, value: i32) {
        let node = Rc::new(RefCell::new(ListNode {
            key,
            value,
            prev: None,
            next: self.get_head(),
        }));
        self.head.replace(node);
    }

    pub fn add_back(&mut self, key: i32, value: i32) {
        let node = Rc::new(RefCell::new(ListNode {
            key,
            value,
            prev: self.get_tail(),
            next: None,
        }));
        self.tail.replace(node);
    }

    pub fn add_front_node(&mut self, node: Rc<RefCell<ListNode>>) {
        // headを取得する
        let head = self.get_head();

        // headが存在すれば、headのprevにnodeを設定する
        if let Some(head) = head.as_ref() {
            head.borrow_mut().prev = Some(node.clone());
        }

        // nodeのprevをNoneに設定する
        node.borrow_mut().prev = None;
        // nodeのnextをheadに設定する
        node.borrow_mut().next = head.clone();

        // self.headにnodeを設定する
        self.head = Some(node);
    }

    pub fn add_back_node(&mut self, node: Rc<RefCell<ListNode>>) {
        let tail = self.get_tail();

        if let Some(tail) = tail.as_ref() {
            tail.borrow_mut().next = Some(node.clone());
        }

        node.borrow_mut().prev = tail.clone();
        node.borrow_mut().next = None;
    }

    pub fn remove(&mut self, node: Rc<RefCell<ListNode>>) {
        // nodeのprevとnextを取得する
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        // prevが存在すれば、prevのnextにnextを設定する
        // nextが存在すれば、nextのprevにprevを設定する
        // nodeがheadなら、self.headにnextを設定する
        // nodeがtailなら、self.tailにprevを設定する
        match (prev, next) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev);
            }
            (Some(prev), None) => {
                prev.borrow_mut().next = None;
                self.tail.replace(prev);
            }
            (None, Some(next)) => {
                next.borrow_mut().prev = None;
                self.head.replace(next);
            }
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
        }
    }

    pub fn move_head(&mut self, node: Rc<RefCell<ListNode>>) {
        // nodeのprevとnextを取得する
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        match (prev, next) {
            (None, Some(_)) => {}
            _ => {
                self.remove(node.clone());
                self.add_front_node(node);
            }
        }
    }

    pub fn move_tail(&mut self, node: Rc<RefCell<ListNode>>) {
        // nodeのprevとnextを取得する
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        match (prev, next) {
            (Some(_), None) => {}
            _ => {
                self.remove(node.clone());
                self.add_back_node(node);
            }
        }
    }
}

// note

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
struct LRUCache {
    cache: HashMap<i32, Rc<RefCell<ListNode>>>,
    lru: DoubleListNode,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cache: HashMap::new(),
            lru: DoubleListNode::new(),
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        println!("cache: {:?}", self.cache);
        println!("lru: {:?}", self.lru);
        println!("capacity: {:?}", self.capacity);
        if let Some(node) = self.cache.get(&key) {
            self.lru.move_head(node.clone());
            node.as_ref().borrow().value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // key, valueが一致するnodeがあれば、そのnodeを先頭に移動する
        if let Some(node) = self.cache.get(&key) {
            self.lru.move_head(node.clone());
            node.as_ref().borrow_mut().value = value;
        } else {
            // なければ、新しいnodeを先頭に追加する
            let node = Rc::new(RefCell::new(ListNode::new(key, value)));
            self.lru.add_front(key, value);
            // capacityを超えていれば、lruの末尾のnodeを削除して、先頭に追加する
            if self.cache.len() > self.capacity {
                let tail = self.lru.get_tail().unwrap();
                self.lru.remove(tail);
            }
        }
    }
}
/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
