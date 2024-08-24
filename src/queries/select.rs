use std::fmt::Display;

use crate::mixin;
use crate::queries::_traits::Query;

pub struct SelectQuery {
    pub parts: Vec<String>,
    pub args: Vec<Box<dyn Display>>,
}

impl Query for SelectQuery {
    fn updated(&mut self, query: String, args: Option<&mut Vec<impl Display>>) -> &Self {
        self.parts.push(query);
        if let Some(args) = args {
            self.args.append(args);
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