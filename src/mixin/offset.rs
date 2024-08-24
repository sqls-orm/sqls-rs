use std::fmt::Display;
use crate::queries::_traits::Query;

pub trait OffsetMixin: Query {
    fn offset(&mut self, offset: impl Into<u128> + Display) -> &Self {
        self.updated(format!("OFFSET %s"), Some(&mut vec![offset]))
    }
}