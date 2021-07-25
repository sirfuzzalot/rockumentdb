use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub enum DataType {
    Null,
    Bool(bool),
    F64(String),
    I64(i64),
    U64(u64),
    String(String),
}

impl Clone for DataType {
    fn clone(&self) -> Self {
        match &self {
            DataType::Null => DataType::Null,
            DataType::Bool(val) => DataType::Bool(*val),
            DataType::String(val) => DataType::String(String::from(val)),
            DataType::F64(val) => DataType::F64(String::from(val)),
            DataType::I64(val) => DataType::I64(*val),
            DataType::U64(val) => DataType::U64(*val),
        }
    }
}

// impl Display for DataType {

// }

pub fn from_json(value: &serde_json::Value) -> DataType {
    match value {
        serde_json::Value::Null => DataType::Null,
        serde_json::Value::Bool(val) => DataType::Bool(*val),
        serde_json::Value::Number(val) => {
            if val.is_f64() {
                DataType::F64(val.to_string())
            } else if val.is_i64() {
                DataType::I64(val.as_i64().unwrap())
            } else {
                DataType::U64(val.as_u64().unwrap())
            }
        }
        serde_json::Value::String(val) => DataType::String(String::from(val)),
        _ => DataType::Null,
    }
}

pub fn to_json(value: &DataType) -> serde_json::Value {
    match value {
        DataType::Null => serde_json::Value::Null,
        DataType::Bool(val) => serde_json::Value::Bool(*val),
        DataType::String(val) => serde_json::to_value(val).unwrap(),
        DataType::F64(val) => {
            let num: f64 = val.parse().unwrap();
            serde_json::to_value(num).unwrap()
        }
        DataType::I64(val) => serde_json::to_value(*val).unwrap(),
        DataType::U64(val) => serde_json::to_value(*val).unwrap(),
    }
}
