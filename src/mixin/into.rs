use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait IntoMixin: Query {
    fn into_(&mut self, table: impl Display) -> &Self {
        let query = format!("INTO {table}");
        let args = None;
        self.updated(query, args)
    }
}