use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Node {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LRUCache {
    cap: usize,
    nodes: Vec<Option<Node>>,
    head: Option<usize>,
    tail: Option<usize>,
    freelist: VecDeque<usize>,
    index: HashMap<i32, usize>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            cap: capacity as usize,
            nodes: vec![],
            head: None,
            tail: None,
            freelist: VecDeque::new(),
            index: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&pos) = self.index.get(&key) {
            let value = self.nodes[pos].as_ref().unwrap().value;
            self.move_to_end(pos);
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&pos) = self.index.get(&key) {
            // Update existing key
            if let Some(node) = self.nodes[pos].as_mut() {
                node.value = value;
            }
            self.move_to_end(pos);
            return;
        }

        // Check if we need to evict
        if self.index.len() >= self.cap {
            if let Some(head_idx) = self.head {
                // Get the key to remove from the index
                let key_to_remove = self.nodes[head_idx].as_ref().unwrap().key;
                self.index.remove(&key_to_remove);
                
                // Update head pointer
                let next = self.nodes[head_idx].as_ref().unwrap().next;
                self.head = next;
                
                if let Some(next_idx) = next {
                    self.nodes[next_idx].as_mut().unwrap().prev = None;
                } else {
                    // If there's no next, the list is now empty
                    self.tail = None;
                }
                
                // Reuse the node
                self.nodes[head_idx] = Some(Node {
                    key,
                    value,
                    prev: self.tail,
                    next: None,
                });
                
                // Add to the end
                if let Some(tail_idx) = self.tail {
                    self.nodes[tail_idx].as_mut().unwrap().next = Some(head_idx);
                    self.tail = Some(head_idx);
                } else {
                    // If there was no tail, this is the only node
                    self.head = Some(head_idx);
                    self.tail = Some(head_idx);
                }
                
                self.index.insert(key, head_idx);
                return;
            }
        }

        // Add new node
        let new_idx = if !self.freelist.is_empty() {
            let idx = self.freelist.pop_front().unwrap();
            self.nodes[idx] = Some(Node {
                key,
                value,
                prev: self.tail,
                next: None,
            });
            idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Some(Node {
                key,
                value,
                prev: self.tail,
                next: None,
            }));
            idx
        };

        // Update linked list
        if let Some(tail_idx) = self.tail {
            self.nodes[tail_idx].as_mut().unwrap().next = Some(new_idx);
        } else {
            self.head = Some(new_idx);
        }
        
        self.tail = Some(new_idx);
        self.index.insert(key, new_idx);
    }

    pub fn move_to_end(&mut self, pos: usize) {
        // If it's already at the end, do nothing
        if Some(pos) == self.tail {
            return;
        }

        let (prev, next) = {
            let node = self.nodes[pos].as_ref().unwrap();
            (node.prev, node.next)
        };

        // Remove from current position
        if let Some(prev_idx) = prev {
            self.nodes[prev_idx].as_mut().unwrap().next = next;
        } else {
            // This was the head
            self.head = next;
        }

        if let Some(next_idx) = next {
            self.nodes[next_idx].as_mut().unwrap().prev = prev;
        }

        // Move to end
        if let Some(tail_idx) = self.tail {
            self.nodes[tail_idx].as_mut().unwrap().next = Some(pos);
            self.nodes[pos].as_mut().unwrap().prev = Some(tail_idx);
        } else {
            // Empty list
            self.nodes[pos].as_mut().unwrap().prev = None;
            self.head = Some(pos);
        }

        self.nodes[pos].as_mut().unwrap().next = None;
        self.tail = Some(pos);
    }
}
