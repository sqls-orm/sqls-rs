use std::fmt::Display;

use crate::mixin;
use crate::queries::_traits::Query;

pub struct DeleteQuery {
    pub parts: Vec<String>,
    pub args: Vec<Box<dyn Display>>,
}

impl Query for DeleteQuery {
    fn updated(&mut self, query: String, args: Option<&mut Vec<impl Display>>) -> &Self {
        self.parts.push(query);
        if let Some(args) = args {
            self.args.append(args);
        }
        self
    }
}

impl mixin::FromMixin for DeleteQuery {}
impl mixin::WhereMixin for DeleteQuery {}
impl mixin::ReturningMixin for DeleteQuery {}