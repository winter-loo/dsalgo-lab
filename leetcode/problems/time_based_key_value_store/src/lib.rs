use std::collections::HashMap;

pub struct TimeMap {
    kvstore: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        TimeMap {
            kvstore: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        // self.kvstore.entry(key).or_insert(vec![(timestamp, value.clone())]).push((timestamp, value));
        if self.kvstore.contains_key(&key) {
            self.kvstore.get_mut(&key).unwrap().push((timestamp, value));
        } else {
            self.kvstore.insert(key, vec![(timestamp, value)]);
        }
    }
    
    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(v) = self.kvstore.get(&key) {
            match v.binary_search_by_key(&timestamp, |&(ts, _)| ts) {
                Ok(i) => return v[i].1.clone(),
                Err(i) if i != 0 => return v[i - 1].1.clone(),
                Err(_) => return "".into(),
            }
        } else {
            return "".into();
        }
    }
}

//
// Your TimeMap object will be instantiated and called as such:
// let obj = TimeMap::new();
// obj.set(key, value, timestamp);
// let ret_2: String = obj.get(key, timestamp);
//
