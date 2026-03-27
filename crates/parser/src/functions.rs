use crate::types::Type;
use crate::value::Value;
use std::collections::HashMap;

pub struct Function {
    pub name: String,
    pub t: Type,
    pub v: Value,
    pub parameters: Option<HashMap<String, String>>,
}
