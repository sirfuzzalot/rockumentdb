use crate::datastore::datatypes::DataType;
use crate::datastore::index::Index;
use crate::datastore::query_proc::{self, QueryResult};
use std::collections::{BTreeMap, HashMap};

pub type Store = BTreeMap<usize, Document>;
pub type Document = HashMap<String, DataType>;
pub type Indices = HashMap<String, Index>;

pub struct Collection {
    pub name: String,
    store: Store,
    last_key: usize,
    indices: HashMap<String, Index>,
}

impl Collection {
    /// Produces a new Collection
    ///
    /// # Arugments
    ///
    /// * `name` - the name of the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collection::Collection;
    /// let mut collection = Collection::new();
    /// ```
    pub fn new(name: String) -> Collection {
        Collection {
            name,
            store: BTreeMap::new(),
            last_key: 0,
            indices: HashMap::new(),
        }
    }

    /// Inserts a document into the collection
    ///
    /// # Arguments
    ///
    /// * `value` - A document in the form of a hashmap with string keys
    ///     and DataType values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collection::Collection;
    ///
    /// let mut collection = Collection::new(String::from("users"));
    ///
    /// let mut document = HashMap::new();
    /// document.insert(
    ///     String::from("username"),
    ///     DataType::String(String::from("johnperry")),
    /// );
    /// document.insert(String::from("age"), DataType::Usize(75usize));
    /// document.insert(String::from("active"), DataType::Bool(true));
    ///
    /// let key = collection.insert(document);
    /// ```
    pub fn insert(&mut self, value: HashMap<String, DataType>) -> usize {
        self.last_key += 1;
        self.store.insert(self.last_key, value);
        self.last_key
        // TODO: Implement Index management functions
    }

    /// Produces the results of a query against the collection
    ///
    /// # Arguments
    ///
    /// * `query` - query statement
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collection::Collection;
    ///
    /// let mut collection = Collection::new(String::from("users"));
    ///
    /// let mut document = HashMap::new();
    /// document.insert(
    ///     String::from("username"),
    ///     DataType::String(String::from("johnperry")),
    /// );
    /// document.insert(String::from("age"), DataType::Usize(75usize));
    /// document.insert(String::from("active"), DataType::Bool(true));
    ///
    /// let key = collection.insert(document);
    ///
    /// let results = collection.find("{username:\"johnperry\"}");
    /// ```
    pub fn find(&self, query: &String) -> QueryResult {
        query_proc::process_query(query, &self.store, &self.indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_new_collection() {
        Collection::new(String::from("users"));
    }

    #[test]
    fn insert_string() {
        let mut collection = Collection::new(String::from("users"));
        let mut document = HashMap::new();
        document.insert(
            String::from("username"),
            DataType::String(String::from("johnperry")),
        );
        document.insert(String::from("age"), DataType::U64(75u64));
        document.insert(String::from("active"), DataType::Bool(true));
        let key = collection.insert(document);
        assert_eq!(1usize, key)
    }

    #[test]
    fn insert_document_2() {
        let mut collection = Collection::new(String::from("users"));
        let mut document = HashMap::new();
        document.insert(
            String::from("username"),
            DataType::String(String::from("johnperry")),
        );
        document.insert(String::from("age"), DataType::U64(75u64));
        document.insert(String::from("active"), DataType::Bool(true));
        let document2 = document.clone();
        collection.insert(document);
        assert_eq!(2usize, collection.insert(document2))
    }

    #[test]
    fn find_document_matching_string() {
        let mut collection = Collection::new(String::from("users"));
        let mut document = HashMap::new();
        document.insert(
            String::from("username"),
            DataType::String(String::from("johnperry")),
        );
        document.insert(String::from("age"), DataType::U64(75u64));
        document.insert(String::from("active"), DataType::Bool(true));
        collection.insert(document.clone());
        let results = match collection.find(&String::from("{username:\"johnperry\"}")) {
            QueryResult::Data(data) => data,
            _ => {
                println!("InvalidCommand");
                Vec::new()
            }
        };
        println!("Results {:?}", &results);
        assert_eq!(&document, results[0])
    }
}
