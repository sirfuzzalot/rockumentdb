# RockumentDB 🤘

A simple document style in-memory database that implements a RESTful HTTP
API possessing limited compatibility with MongoDB query syntax.

---

## Roadmap

### 2.x.x

| Status | Task                            |
| :----: | :------------------------------ |
|   -    | Build out query system          |
|   ✔️   | Further define datastore model  |
|   ✔️   | Implement limited test coverage |

### 1.x.x

| Status | Task                                   |
| :----: | :------------------------------------- |
|   ✔️   | Create architectural component diagram |
|   ✔️   | Implement the data store component     |
|   ✔️   | Implement the HTTP API component       |

---

## API Reference

RockumentDB implements a RESTful HTTP API.

- [version](#version)
- [insert](#insert)
- [find](#find)

### version

| Method | Path | Content-Type |
| :----: | :--- | :----------- |
|  GET   | /    | text/plain   |

#### Response (200)

```
RockumentDB 2.0.0-alpha
```

### insert

| Method | Path                 | Content-Type     |
| :----: | :------------------- | :--------------- |
|  POST  | /api/v2/{collection} | application/json |

#### Request

```json
[
  {
    "username": "johnperry",
    "email": "johnperry@example.com",
    "first_name": "John",
    "last_name": "Perry",
    "age": 75
  },
  {
    "username": "louiswu",
    "email": "louiswu@example.com",
    "first_name": "Louis",
    "last_name": "Wu",
    "age": 200
  }
]
```

#### Response (201)

A list the newly created document Id's

```json
[1, 2]
```

### find

| Method | Path                               | Content-Type     |
| :----: | :--------------------------------- | :--------------- |
|  GET   | /api/v2/{collection}?query={query} | application/json |

#### Request

URL encoded query param using MongoDB style find query.

```
http://{{server}}/api/v2/test?query={username:"johnperry"}
```

#### Response (200)

```json
[
  {
    "username": "johnperry",
    "email": "johnperry@example.com",
    "first_name": "John",
    "last_name": "Perry",
    "age": 75
  }
]
```

#### Response (400)

Invalid queries return 400 and an empty array.

```json
[]
```

#### Response (500)

Internal server issues return 500 and an empty array

```json
[]
```

## Contributing

### Tests

RockumentDB has a test suite written in Rust and Python. Unittests are
kept in their respective modules and are in Rust. HTTP API tests are
written in Python and are located in the `tests` directory.

#### Unittests

I highly recommend the [Cargo Watch](https://crates.io/crates/cargo-watch) plugin for a TDD workflow.

```
cargo install cargo-watch
cargo watch -x "test -- --nocapture"
```

or on its own

```
cargo test
```

If you use VSCode as your editor you can create a task that runs your tests everytime you save. `.vscode/tasks.json`

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "type": "shell",
      "command": "cargo watch --watch src --poll --postpone --quiet --clear -x \"test -- --nocapture\"",
      "problemMatcher": ["$rustc"],
      "group": {
        "kind": "test",
        "isDefault": true
      },
      "label": "Rust: cargo watch test",
      "presentation": {
        "reveal": "always",
        "clear": true
      },
      "runOptions": {
        "runOn": "folderOpen"
      }
    }
  ]
}
```

#### HTTP API Tests

To test the HTTP API we use `pytest`. It's a powerful testing framework
that's fast to iterate with. You will need Python 3.6+ to run these
tests.

bash

```bash
python -m venv venv
source venv/bin/activate
python -m pip install -r requirements.txt
```

pwsh

```powershell
python -m venv venv
.\venv\Scripts\activate
python -m pip install -r requirements.txt
```

```bash
python -m pytest -vs tests
```

## Resources

- RockumentDB's Rust Docs - `cargo doc --open`
- [Rocket Crate](https://rocket.rs/)
- [Serde](https://serde.rs/)
- [Serde JSON](https://docs.serde.rs/serde_json/)
