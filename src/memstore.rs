use crate::errors::{KeyNotFoundError, RangeError};
use std::collections::{BTreeMap, HashMap};

pub struct MemStore {
    store: BTreeMap<usize, String>,
    last_key: usize,
}

impl MemStore {
    pub fn new() -> MemStore {
        MemStore {
            store: BTreeMap::new(),
            last_key: 0,
        }
    }

    pub fn get(&self, key: usize) -> Option<&String> {
        println!("GET - {}", &key);
        self.store.get(&key)
    }

    pub fn post(&mut self, value: String) -> usize {
        println!("POST - {}", &value);
        self.last_key += 1;
        self.store.insert(self.last_key, value);
        println!("LAST KEY: {}", &self.last_key);
        self.last_key
    }

    pub fn put(&mut self, key: usize, value: String) -> Result<(), KeyNotFoundError> {
        println!("PUT - {} {}", &key, &value);
        if !self.store.contains_key(&key) {
            return Err(KeyNotFoundError);
        };
        self.store.insert(key, value);
        Ok(())
    }

    pub fn delete(&mut self, key: usize) -> Result<(), KeyNotFoundError> {
        println!("DELETE - {}", &key);
        if !self.store.contains_key(&key) {
            return Err(KeyNotFoundError);
        };
        self.store.remove(&key);
        Ok(())
    }

    pub fn list(&self, start: usize, count: usize) -> Result<HashMap<usize, String>, RangeError> {
        if self.store.is_empty() {
            return Ok(HashMap::new());
        };

        let mut final_key = &0usize;
        for k in self.store.keys() {
            final_key = k;
        }

        if &start > final_key {
            return Err(RangeError);
        };

        let computed_count = if count == 0 { 99 } else { count };
        println!("LIST - {} - {}", start, computed_count);

        let mut results = HashMap::new();
        for (index, (&key, value)) in self.store.range(start..).enumerate() {
            if index >= computed_count {
                break;
            };
            // println!("| {} | {} |", key, &value);
            results.insert(key, value.clone());
        }

        Ok(results)
    }
}
