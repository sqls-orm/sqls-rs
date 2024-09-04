use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait FromMixin: Query {
    fn from<T>(mut self, table: T) -> Self
    where
        T: Display + 'static,
    {
        let query = format!("FROM {table}");
        let args = Vec::<T>::default();
        self.updated(query, args)
    }
}