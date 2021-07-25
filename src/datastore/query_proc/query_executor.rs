use crate::datastore::collection::{Indices, Store};
use crate::datastore::query_proc::query_ingestor::Instructions;
use crate::datastore::query_proc::QueryResult;
use std::collections::HashSet;

/// Produces the results of the query operations
///
/// # Arguments
///
/// * `instructions` - a list of Instructions to execute
/// * `store` - the Collection's store to operate on
/// * `indices` - mapping of the collection's Index structs to their field name.
///
/// # Example
///
/// ```rust
/// let ops = vec![
///    Instructions::Equal(
///        String::from("username"),
///        DataType::String(String::from("johnperry"))
///    ),
///    Instructions::Equal(
///        String::from("email"),
///        DataType::String(String::from("johnperry@example.com"))
///    )
/// ];
/// let results = process_instructions(ops, store, indices);
/// ```
pub fn process_instructions<'a>(
    instructions: Vec<Instructions>,
    store: &'a Store,
    indices: &Indices,
) -> QueryResult<'a> {
    let mut is_first_equal = true;
    let mut working_results = HashSet::new();
    for instruction in instructions {
        match instruction {
            Instructions::Equal(field, value) => {
                if let Some(index) = indices.get(&field) {
                    if let Some(ids) = index.search(&value) {
                        if is_first_equal {
                            for id in ids.into_iter() {
                                working_results.insert(id.clone());
                            }
                        } else {
                            let mut next_set = HashSet::new();
                            for id in ids.into_iter() {
                                next_set.insert(id.clone());
                            }
                            let mut compared_set = HashSet::new();
                            for id in working_results.intersection(&next_set).into_iter() {
                                compared_set.insert(id.clone());
                            }
                            working_results = compared_set;
                        }
                    }
                } else {
                    for (id, document) in store.into_iter() {
                        if let Some(found_value) = document.get(&field) {
                            if found_value == &value {
                                working_results.insert(id.clone());
                            }
                        }
                    }
                }
                is_first_equal = false;
            }
            _ => {}
        }
    }
    gather_documents(working_results.into_iter().collect(), store)
}

/// Produces the documents associated with the ids
///
/// # Arguments
///
/// * `ids` - a list of document ids
/// * `store` - the Collection's store to operate on
///
fn gather_documents<'a>(ids: Vec<usize>, store: &'a Store) -> QueryResult<'a> {
    let mut results = Vec::new();
    for id in ids {
        if let Some(document) = store.get(&id) {
            results.push(document)
        } else {
            return QueryResult::InvalidIdError;
        }
    }
    QueryResult::Data(results)
}
