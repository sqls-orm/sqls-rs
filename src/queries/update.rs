use std::fmt::Display;

use crate::mixin;
use crate::queries::_traits::Query;

pub struct UpdateQuery {
    pub parts: Vec<String>,
    pub args: Vec<Box<dyn Display>>,
}

impl Query for UpdateQuery {
    fn updated(&mut self, query: String, args: Option<&mut Vec<impl Display>>) -> &Self {
        self.parts.push(query);
        if let Some(args) = args {
            self.args.append(args);
        }
        self
    }
}

impl mixin::WhereMixin for UpdateQuery {}
impl mixin::ReturningMixin for UpdateQuery {}