use std::fmt::Display;

use crate::query::Query;

pub fn insert() -> Query {
    let mut parts = Vec::<String>::default();
    parts.push("INSERT".to_string());

    Query::new(parts, Vec::new())
}

pub fn select<T>(columns: Vec<T>) -> Query
where
    T: Display + 'static,
{
    let part = columns
        .iter()
        .map(|col| format!("{col}"))
        .collect::<Vec<String>>()
        .join(", ");
    let mut parts = Vec::<String>::default();
    parts.push(format!("SELECT {part}"));

    Query::new(parts, Vec::new())
}

pub fn delete() -> Query {
    let mut parts = Vec::<String>::default();
    parts.push("DELETE".to_string());

    Query::new(parts, Vec::new())
}

pub fn update<'a, T>(table: T) -> Query
where
    T: Display,
{
    let mut parts = Vec::<String>::default();
    parts.push(format!("UPDATE {table}"));

    Query::new(parts, Vec::new())
}

pub fn upsert() -> Query {
    let mut parts = Vec::<String>::default();
    parts.push(format!("REPLACE"));

    Query::new(parts, Vec::new())
}