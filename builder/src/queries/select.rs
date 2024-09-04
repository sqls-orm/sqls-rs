use std::fmt::Display;

use crate::mixin;
use crate::queries::_traits::Query;

pub struct SelectQuery {
    pub parts: Vec<String>,
    pub args: Vec<Box<dyn Display>>,
}

impl Query for SelectQuery {
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

impl mixin::FromMixin for SelectQuery {}
impl mixin::JoinMixin for SelectQuery {}
impl mixin::WhereMixin for SelectQuery {}
impl mixin::OrderByMixin for SelectQuery {}
impl mixin::LimitMixin for SelectQuery {}
impl mixin::OffsetMixin for SelectQuery {}