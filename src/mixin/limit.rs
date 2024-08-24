use std::fmt::Display;
use crate::queries::_traits::Query;

pub trait LimitMixin: Query {
    fn limit(&mut self, limit: impl Into<u128> + Display) -> &Self {
        self.updated(format!("LIMIT %s"), Some(&mut vec![limit]))
    }
}