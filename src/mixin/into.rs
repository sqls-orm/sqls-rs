use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait IntoMixin: Query {
    fn into_(&mut self, table: impl Display) -> &Self {
        self.updated(format!("INTO {table}"), None::<&mut Vec<_>>)
    }
}