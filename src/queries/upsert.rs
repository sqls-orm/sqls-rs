use std::fmt::Display;

use crate::mixin;
use crate::queries::_traits::Query;

pub struct UpsertQuery {
    pub parts: Vec<String>,
    pub args: Vec<Box<dyn Display>>,
}

impl Query for UpsertQuery {
    fn updated<T>(&mut self, query: String, args: Vec<T>) -> &Self
    where
        T: Display + 'static,
    {
        self.parts.push(query);
        for e in args {
            self.args.push(Box::new(e))
        }
        self
    }
}

impl mixin::IntoMixin for UpsertQuery {}
impl mixin::ValuesMixin for UpsertQuery {}
impl mixin::ReturningMixin for UpsertQuery {}