use std::fmt::Display;
use crate::queries::_traits::Query;

pub trait LimitMixin: Query {
    fn limit<T>(&mut self, limit: T) -> &Self
    where
        T: Into<u128> + Display + 'static,
    {
        let query = format!("LIMIT %s");
        let args = vec![limit];
        self.updated(query, args)
    }
}