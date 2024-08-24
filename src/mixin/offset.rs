use std::fmt::Display;
use crate::queries::_traits::Query;

pub trait OffsetMixin: Query {
    fn offset<T>(&mut self, offset: T) -> &Self
    where
        T: Into<u128> + Display,
    {
        let query = format!("OFFSET %s");
        let args = Some(&mut vec![offset]);
        self.updated(query, args)
    }
}