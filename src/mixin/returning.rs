use std::fmt::Display;

use crate::queries::_traits::Query;
use crate::utils;

pub trait ReturningMixin: Query {
    fn returning<T>(&mut self, columns: Vec<T>) -> &Self
    where
        T: Display + 'static,
    {
        let query = utils::join(columns);
        let args = Vec::<T>::default();
        self.updated(query, args)
    }
}