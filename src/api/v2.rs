use crate::datastore::datatypes;
use crate::datastore::query_proc::QueryResult;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

#[get("/<collection_name>?<query>")]
pub fn find(
    collection_name: String,
    query: Option<String>,
    db: &rocket::State<crate::SafeMemStore>,
) -> status::Custom<Json<Vec<BTreeMap<String, Value>>>> {
    let mutex = db.inner();
    let collection = mutex.lock().unwrap();
    let search = match query {
        Some(q) => q,
        None => String::from("{}"),
    };
    println!("FIND: Collection - {} - {}", &collection_name, &search);

    match collection.find(&search) {
        QueryResult::Data(values) => {
            let mut results = Vec::new();
            for doc in values.into_iter() {
                let mut converted_doc = BTreeMap::new();
                for (field, value) in doc.iter() {
                    converted_doc.insert(field.clone(), datatypes::to_json(value));
                }
                results.push(converted_doc);
            }
            status::Custom(Status::Ok, Json(results))
        }
        QueryResult::InvalidQueryError => status::Custom(Status::BadRequest, Json(Vec::new())),
        QueryResult::InvalidIdError => {
            status::Custom(Status::InternalServerError, Json(Vec::new()))
        }
    }
}

/// Insert a list of documents into a collection
///
/// # Arguments
///
/// * `collection_name` - the collection to insert the documents into
/// * `values` - HTTP request body containing a list of documents
/// * `db` - thread-safe collection
///
/// # Example
///
/// ```json
/// # values
/// [
///   {
///     "username": "johnperry",
///     "email": "johnperry@example.com",
///     "first_name": "John",
///     "last_name": "Perry",
///     "age": 75
///   },
///   {
///     "username": "louiswu",
///     "email": "louiswu@example.com",
///     "first_name": "Louis",
///     "last_name": "Wu",
///     "age": 200
///   }
/// ]
/// ```
#[post("/<collection_name>", format = "json", data = "<values>")]
pub fn insert(
    collection_name: String,
    values: Json<Vec<HashMap<String, Value>>>,
    db: &rocket::State<crate::SafeMemStore>,
) -> status::Custom<Json<Vec<usize>>> {
    let mut collection = db.lock().unwrap();

    let mut ids = Vec::new();
    for doc in values.into_inner().into_iter() {
        let mut converted_doc = HashMap::new();
        for (field, value) in doc.iter() {
            converted_doc.insert(field.clone(), datatypes::from_json(value));
        }
        ids.push(collection.insert(converted_doc))
    }
    println!(
        "INSERT: Collection - {} - {} documents",
        &collection_name,
        ids.len()
    );

    status::Custom(Status::Created, Json(ids))
}
