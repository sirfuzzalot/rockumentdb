use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Index {
    pub tree: BTreeMap<u64, Vec<usize>>,
}

impl Index {
    /// Produces a new Index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use index::Index;
    /// let mut index = Index::new();
    /// ```
    pub fn new() -> Index {
        Index {
            tree: BTreeMap::new(),
        }
    }

    /// Adds a key and its associated id to the index.
    ///
    /// # Arguments
    ///
    /// * `key` - any hashable type that you want to make searchable
    /// * `value` - usize id of the document matching the key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use index::Index;
    /// let mut index = Index::new();
    /// let key = String::from("John Perry");
    /// let value: usize = 1;
    /// index.insert(&key, value);
    /// ```
    pub fn insert<T: Hash>(&mut self, key: &T, value: usize) -> u64 {
        let hash_key = calculate_hash(key);
        let list = match self.tree.get_mut(&hash_key) {
            Some(existing_values) => {
                existing_values.push(value);
                existing_values.to_vec()
            }
            None => vec![value],
        };
        self.tree.insert(hash_key, list);
        hash_key
    }

    /// Produces all ids that match the search key
    ///
    /// # Arguments
    ///
    /// * `key` - any hashable type that you want to search the index for
    ///
    /// # Examples
    ///
    /// ```rust
    /// use index::Index;
    /// let name_index = Index::new();
    /// let results = name_index.search("John");
    /// match results {
    ///     Some(val) => val,
    ///     None => vec![]
    /// }
    /// ```
    pub fn search<T: Hash>(&self, key: &T) -> Option<&Vec<usize>> {
        let hash_key = calculate_hash(key);
        self.tree.get(&hash_key)
    }
}

impl Default for Index {
    fn default() -> Self {
        Index::new()
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_new_key() {
        let mut index = Index::new();
        let key = String::from("John Perry");
        let value: usize = 1;
        assert_eq!(9548898927525612867, index.insert(&key, value))
    }

    #[test]
    fn insert_existing_key() {
        let mut index = Index::new();
        let key = String::from("John Perry");
        let value: usize = 1;
        let value2: usize = 2;
        index.insert(&key, value);
        assert_eq!(9548898927525612867, index.insert(&key, value2));
    }

    #[test]
    fn search_missing_key() {
        let index = Index::new();
        let key = String::from("John Perry");
        assert_eq!(None, index.search(&key));
    }

    #[test]
    fn search_existing_key() {
        let mut index = Index::new();
        let key = String::from("John Perry");
        let value: usize = 1;
        index.insert(&key, value);
        assert_eq!(Some(&vec![value]), index.search(&key));
    }

    #[test]
    fn search_existing_key_2() {
        let mut index = Index::new();
        let key = String::from("John Perry");
        let value: usize = 1;
        let value2: usize = 2;
        index.insert(&key, value);
        index.insert(&key, value2);
        assert_eq!(Some(&vec![value, value2]), index.search(&key));
    }
}
