use rocket::data::Data;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::collections::BTreeMap;
use std::io::Read;

#[get("/<table>/<id>")]
pub fn get(
    table: String,
    id: usize,
    db: rocket::State<crate::TSMemStore>,
) -> status::Custom<Json<serde_json::Value>> {
    println!("GET - {}", &table);
    let datastore = db.lock().unwrap();
    let results = match datastore.get(id) {
        Some(v) => status::Custom(Status::Ok, Json(serde_json::from_str(&v.clone()).unwrap())),
        None => status::Custom(
            Status::NotFound,
            Json(serde_json::from_str("{\"error\": \"Key Not Found\"}").unwrap()),
        ),
    };
    results
}

#[get("/<table>")]
pub fn list(
    table: String,
    db: rocket::State<crate::TSMemStore>,
) -> Json<BTreeMap<usize, serde_json::Value>> {
    println!("LIST - {}", &table);
    let datastore = db.lock().unwrap();
    let results = datastore.list(1, 0).unwrap();
    let mut map = BTreeMap::new();
    for (key, value) in results {
        map.insert(key, serde_json::from_str(&value).unwrap());
    }
    Json(map)
}

#[post("/<table>", format = "json", data = "<value>")]
pub fn post(
    table: String,
    value: Data,
    db: rocket::State<crate::TSMemStore>,
) -> status::Custom<Json<serde_json::Value>> {
    println!("POST - {}", &table);
    let mut datastore = db.lock().unwrap();

    let mut stream = value.open();
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();

    let results = datastore.post(buf);

    status::Custom(
        Status::Created,
        Json(serde_json::from_str(&format!("{{\"id\": {}}}", results)).unwrap()),
    )
}

#[put("/<table>/<key>", format = "json", data = "<value>")]
pub fn put(
    table: String,
    key: usize,
    value: Data,
    db: rocket::State<crate::TSMemStore>,
) -> status::Custom<Json<serde_json::Value>> {
    println!("PUT - {}", &table);
    let mut datastore = db.lock().unwrap();

    let mut stream = value.open();
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();

    match datastore.put(key, buf) {
        Ok(_) => status::Custom(
            Status::Ok,
            Json(serde_json::from_str(&format!("{{\"id\":{} }}", &key)).unwrap()),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            Json(serde_json::from_str("{\"error\": \"Key Not Found\"}").unwrap()),
        ),
    }
}

#[delete("/<table>/<key>")]
pub fn delete(
    table: String,
    key: usize,
    db: rocket::State<crate::TSMemStore>,
) -> status::Custom<Json<serde_json::Value>> {
    println!("Delete - {}", &table);
    let mut datastore = db.lock().unwrap();
    match datastore.delete(key) {
        Ok(_) => status::Custom(
            Status::Ok,
            Json(serde_json::from_str(&format!("{{\"id\":{} }}", &key)).unwrap()),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            Json(serde_json::from_str("{\"error\": \"Key Not Found\"}").unwrap()),
        ),
    }
}
