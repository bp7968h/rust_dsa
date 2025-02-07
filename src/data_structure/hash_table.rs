use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type Key = String;
type Value = String;

#[derive(Debug, Clone)]
struct Node {
    key: Key,
    value: Value,
    next: Option<Box<Node>>,
}

pub struct HashTable {
    buckets: Vec<Option<Box<Node>>>,
    size: usize,
}

impl HashTable {
    pub fn new(size: usize) -> HashTable {
        HashTable {
            buckets: vec![None; size],
            size,
        }
    }

    fn hash(key: &str, size: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % size
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        let index = Self::hash(&key, self.size);
        let new_node = Box::new(Node {
            key,
            value,
            next: self.buckets[index].take(),
        });
        self.buckets[index] = Some(new_node);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        let index = Self::hash(key, self.size);
        let mut current = self.buckets[index].as_ref();

        while let Some(node) = current {
            if node.key == key {
                return Some(&node.value);
            }
            current = node.next.as_ref();
        }
        None
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        let index = Self::hash(key, self.size);
        let mut current = self.buckets[index].take();
        let mut prev: Option<Box<Node>> = None;

        while let Some(mut node) = current {
            if node.key == key {
                if let Some(mut prev_node) = prev {
                    prev_node.next = node.next.take();
                    self.buckets[index] = Some(prev_node);
                } else {
                    self.buckets[index] = node.next.take();
                }
                return Some(node.value);
            }
            prev = Some(node);
            current = prev.as_mut().unwrap().next.take();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table() {
        let mut table = HashTable::new(10);

        table.insert("key1".to_string(), "value1".to_string());
        table.insert("key2".to_string(), "value2".to_string());

        assert_eq!(table.get("key1"), Some(&"value1".to_string()));
        assert_eq!(table.get("key2"), Some(&"value2".to_string()));

        table.remove("key1");

        assert_eq!(table.get("key1"), None);
        assert_eq!(table.get("key2"), Some(&"value2".to_string()));
    }
}
