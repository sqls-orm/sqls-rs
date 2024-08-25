use std::fmt::Display;

use crate::queries::_traits::Query;
use crate::types::Column;

pub trait JoinMixin: Query {
    fn join<T>(&mut self, table: T) -> &Self
    where
        T: Display + 'static,
    {
        let query = format!("JOIN {table}");
        let args = Vec::<T>::default();
        self.updated(query, args)
    }

    fn on(&mut self, on: Column) -> &Self {
        let query = format!("ON {on}");
        let args = on.args;
        self.updated(query, args)
    }
}