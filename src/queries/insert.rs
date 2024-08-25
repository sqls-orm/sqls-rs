use std::fmt::Display;

use crate::{mixin, utils};
use crate::queries::_traits::Query;
use crate::types::Column;

pub struct InsertQuery {
    pub parts: Vec<String>,
    pub args: Vec<Box<dyn Display>>,
}

impl Query for InsertQuery {
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

impl InsertQuery {
    pub fn on_duplicate(&mut self) -> &Self {
        let query = "ON DUPLICATE".to_string();
        let args = utils::args();
        self.updated(query, args)
    }

    pub fn update(&mut self, values: Vec<Column>) -> &Self {
        let (query, args) = utils::parse(values);
        self.updated(query, args)
    }

    pub fn ignore(&mut self) -> &Self {
        let query = "IGNORE".to_string();
        let args = utils::args();
        self.updated(query, args)
    }
}

impl mixin::IntoMixin for InsertQuery {}
impl mixin::ValuesMixin for InsertQuery {}
impl mixin::ReturningMixin for InsertQuery {}