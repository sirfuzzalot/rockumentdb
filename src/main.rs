#![feature(proc_macro_hygiene, decl_macro)]
mod api;
mod errors;
mod memstore;

use crate::memstore::MemStore;
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

type TSMemStore = Mutex<MemStore>;

#[get("/")]
fn index() -> &'static str {
    "MethodDB 1.0.0-alpha"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount(
            "/api/v1",
            routes![
                api::v1::get,
                api::v1::list,
                api::v1::post,
                api::v1::put,
                api::v1::delete
            ],
        )
        .manage(TSMemStore::new(MemStore::new()))
        .launch();
}
