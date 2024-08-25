use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait IntoMixin: Query {
    fn into_<T>(&mut self, table: T) -> &Self
    where
        T: Display + 'static,
    {
        let query = format!("INTO {table}");
        let args = Vec::<T>::default();
        self.updated(query, args)
    }
}