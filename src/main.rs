mod api;
mod datastore;

use crate::datastore::collection::Collection;
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

type SafeMemStore = Mutex<Collection>;

#[get("/")]
fn version() -> &'static str {
    "RockumentDB 2.0.0-alpha"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![version])
        .mount("/api/v2", routes![api::v2::find, api::v2::insert])
        .manage(SafeMemStore::new(Collection::new(String::from("test"))))
}
