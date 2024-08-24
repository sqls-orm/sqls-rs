use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait FromMixin: Query {
    fn from(&mut self, table: impl Display) -> &Self {
        self.updated(format!("FROM {table}"), None::<&mut Vec<_>>)
    }
}