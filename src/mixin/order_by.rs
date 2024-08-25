use std::fmt::Display;

use crate::queries::_traits::Query;
use crate::utils;

pub trait OrderByMixin: Query {
    fn order_by<T>(&mut self, columns: Vec<T>) -> &Self
    where
        T: Display + 'static,
    {
        let query = format!("ORDER BY {}", utils::join(columns));
        let args = Vec::<T>::default();
        self.updated(query, args)
    }
}