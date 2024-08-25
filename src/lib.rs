use std::fmt::Display;
use crate::types::Column;

pub mod models;
pub mod mixin;
mod queries;
pub mod utils;
pub mod types;

#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}

pub fn insert() -> queries::InsertQuery {
    queries::InsertQuery {
        parts: vec![],
        args: vec![],
    }
}

pub fn select<'a>(columns: Vec<Column>) -> queries::SelectQuery {
    let (query, args) = utils::parse(columns);
    queries::SelectQuery {
        parts: vec![query],
        args: args,
    }
}

pub fn delete() -> queries::DeleteQuery {
    let query = format!("DELETE");
    let args = utils::args();
    queries::DeleteQuery {
        parts: vec![query],
        args: args,
    }
}

pub fn update<T>(table: T) -> queries::UpdateQuery
where
    T: Display,
{
    let query = format!("UPDATE {table}");
    let args = utils::args();
    queries::UpdateQuery {
        parts: vec![query],
        args: args,
    }
}

pub fn upsert() -> queries::UpsertQuery {
    let query = format!("REPLACE");
    let args = utils::args();
    queries::UpsertQuery {
        parts: vec![query],
        args: args,
    }
}