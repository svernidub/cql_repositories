#[derive(Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
}

use Value::*;

impl Value {
    pub fn to_cql(&self) -> String {
        match self {
            Int(x) => format!("{}", x),
            Float(x) => format!("{}", x),
            Boolean(x) => format!("{}", x),
            Text(x) => format!("\"{}\"", x),
        }
    }
}