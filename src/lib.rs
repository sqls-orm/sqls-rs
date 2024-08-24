use std::fmt::Display;

mod mixin;
mod vendors;
mod queries;
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

pub fn select(columns: Vec<impl Display>) -> queries::SelectQuery {
    queries::SelectQuery {
        parts: columns.join(", "),
        args: vec![],
    }
}

pub fn delete() -> queries::DeleteQuery {
    queries::DeleteQuery {
        parts: vec![],
        args: vec![],
    }
}

pub fn update() -> queries::UpdateQuery {
    queries::UpdateQuery {
        parts: vec![],
        args: vec![],
    }
}

pub fn upsert() -> queries::UpsertQuery {
    queries::UpsertQuery {
        parts: vec![],
        args: vec![],
    }
}