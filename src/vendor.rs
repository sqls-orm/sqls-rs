use std::fmt::Display;
use std::sync::{Arc, Mutex};

use crate::column::Column;
use crate::sql::Query;

pub fn insert() -> Query {
    let mut parts = Vec::<String>::default();
    parts.push("INSERT".to_string());

    let parts = Arc::new(Mutex::new(parts));
    let args = Arc::new(Mutex::new(Vec::new()));

    Query { parts, args }
}

pub fn select(columns: Vec<&Column>) -> Query {
    let mut parts = Vec::<String>::default();
    let mut part = Vec::<String>::default();
    for col in columns {
        part.push(col.val.lock().unwrap().to_string());
    }
    parts.push(format!("SELECT {}", part.join(", ")));

    let parts = Arc::new(Mutex::new(parts));
    let args = Arc::new(Mutex::new(Vec::new()));

    Query { parts, args }
}

pub fn delete() -> Query {
    let mut parts = Vec::<String>::default();
    parts.push("DELETE".to_string());

    let parts = Arc::new(Mutex::new(parts));
    let args = Arc::new(Mutex::new(Vec::new()));

    Query { parts, args }
}

pub fn update<'a, T>(table: T) -> Query
where
    T: Display,
{
    let mut parts = Vec::<String>::default();
    parts.push(format!("UPDATE {table}"));

    let parts = Arc::new(Mutex::new(parts));
    let args = Arc::new(Mutex::new(Vec::new()));

    Query { parts, args }
}

pub fn upsert() -> Query {
    let mut parts = Vec::<String>::default();
    parts.push(format!("REPLACE"));

    let parts = Arc::new(Mutex::new(parts));
    let args = Arc::new(Mutex::new(Vec::new()));

    Query { parts, args }
}