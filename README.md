# MethodDB

A simple Rustlang key-value in memory datastore, supporting a subset of
HTTP verbs as commands.

---

## Roadmap

### 2.x.x

| Status | Task                            |
| :----: | :------------------------------ |
|   -    | Build out query system          |
|   -    | Further define datastore model  |
|   -    | Implement limited test coverage |

### 1.x.x

| Status | Task                                   |
| :----: | :------------------------------------- |
|   ✔️   | Create architectural component diagram |
|   ✔️   | Implement the data store component     |
|   ✔️   | Implement the HTTP API component       |

---

## API Reference

MethodDB implements a RESTful HTTP API. Note the table path segment is
currently unused by input is required, so any arbitrary URL-safe string
is valid.

- [create](#create)
- [read one](#read-one)
- [replace](#replace)
- [delete](#delete)
- [list](#list)

### Create

| Method | Path            | Content-Type     |
| :----: | :-------------- | :--------------- |
|  POST  | /api/v1/{table} | application/json |

#### Request

```json
{
  "field": "value",
  "test": 1,
  "depth": {
    "here": ["array", "array2"]
  }
}
```

#### Response (201)

```json
{
  "id": 103
}
```

### Read One

| Method | Path                 | Content-Type     |
| :----: | :------------------- | :--------------- |
|  GET   | /api/v1/{table}/{id} | application/json |

#### Response (200)

```json
{
  "datetime": "2020-06-13T13:01:00-07:00",
  "username": "sirfuzzalot"
}
```

#### Response (404)

```json
{
  "error": "Key Not Found"
}
```

### Replace

| Method | Path                 | Content-Type     |
| :----: | :------------------- | :--------------- |
|  PUT   | /api/v1/{table}/{id} | application/json |

#### Response (200)

```json
{
  "id": 104
}
```

#### Response (404)

```json
{
  "error": "Key Not Found"
}
```

### Delete

| Method | Path                 | Content-Type     |
| :----: | :------------------- | :--------------- |
| DELETE | /api/v1/{table}/{id} | application/json |

#### Response (200)

```json
{
  "id": 104
}
```

#### Response (404)

```json
{
  "error": "Key Not Found"
}
```

### List

| Method | Path            | Content-Type     |
| :----: | :-------------- | :--------------- |
|  GET   | /api/v1/{table} | application/json |

#### Response (200)

```json
{
  "1": {
    "datetime": "2020-06-13T13:01:00-07:00",
    "username": "sirfuzzalot"
  },
  "2": {
    "datetime": "2020-06-13T13:01:00-07:00",
    "username": "sirfuzzalot"
  },
  "3": {
    "datetime": "2020-06-13T13:01:00-07:00",
    "username": "sirfuzzalot"
  },
  "4": {
    "datetime": "2020-06-13T13:01:00-07:00",
    "username": "sirfuzzalot"
  },
  "5": {
    "datetime": "2020-06-13T13:01:00-07:00",
    "username": "sirfuzzalot"
  }
}
```

## Resources

- [Rocket](https://rocket.rs/)
- [Binary Tree Map](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
