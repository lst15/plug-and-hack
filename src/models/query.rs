// src/models/query.rs
pub trait Query {
    fn to_string(&self) -> String;
}