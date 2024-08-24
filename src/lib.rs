use std::fmt::Display;
use crate::types::{Args, Query};

mod mixin;
mod vendors;
mod queries;
pub mod types;

pub fn insert() -> queries::InsertQuery {
    queries::InsertQuery {}
}

pub fn select(columns: Vec<impl Display>) -> queries::SelectQuery {
    queries::SelectQuery {
        parts: columns.join(", "),
        args: vec![],
    }
}

pub fn delete() -> queries::DeleteQuery {
    queries::DeleteQuery {}
}

pub fn update() -> queries::UpdateQuery {
    queries::UpdateQuery {}
}

pub fn upsert() -> queries::UpsertQuery {
    queries::UpsertQuery {}
}