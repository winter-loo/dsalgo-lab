#![allow(unused)]
// MinStack implementation
// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time
pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack { stack: vec![] }
    }

    pub fn push(&mut self, val: i32) {
        let m = if let Some((_, m)) = self.stack.last() {
            (*m).min(val)
        } else {
            val
        };
        self.stack.push((val, m));
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().copied().unwrap_or((0, 0)).0
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().copied().unwrap_or((0, 0)).1
    }
}
