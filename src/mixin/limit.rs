use std::fmt::Display;
use crate::queries::_traits::Query;

pub trait LimitMixin: Query {
    fn limit<T>(&mut self, limit: T) -> &Self
    where
        T: Into<u128> + Display,
    {
        let query = format!("LIMIT %s");
        let args = Some(&mut vec![limit]);
        self.updated(query, args)
    }
}