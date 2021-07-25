mod query_executor;
mod query_ingestor;

use crate::datastore::collection::{Document, Indices, Store};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum QueryResult<'a> {
    Data(Vec<&'a Document>),
    InvalidQueryError,
    InvalidIdError,
}

/// Produces the results of a query against a Collection
///
/// # Arguments
///
/// * `command` - query string
/// * `store` - the Collection's store to operate on
/// * `indices` - mapping of the collection's Index structs to their field name.
///
pub fn process_query<'a>(command: &String, store: &'a Store, indices: &Indices) -> QueryResult<'a> {
    match query_ingestor::ingest(command) {
        Ok(instructions) => query_executor::process_instructions(instructions, store, indices),
        Err(e) => e,
    }
}
