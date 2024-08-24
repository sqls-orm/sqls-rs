use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait FromMixin: Query {
    fn from(&mut self, table: impl Display) -> &Self {
        let query = format!("FROM {table}");
        let args = None;
        self.updated(query, args)
    }
}