use std::fmt::Display;

use crate::mixin;
use crate::queries::_traits::Query;

pub struct UpdateQuery {
    pub parts: Vec<String>,
    pub args: Vec<Box<dyn Display>>,
}

impl Query for UpdateQuery {
    fn updated<T>(&mut self, query: String, args: Vec<T>) -> &mut Self
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

impl mixin::WhereMixin for UpdateQuery {}
impl mixin::ReturningMixin for UpdateQuery {}