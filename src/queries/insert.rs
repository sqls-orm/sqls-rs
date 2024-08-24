use std::fmt::Display;

use crate::mixin;
use crate::queries::_traits::{ColToVal, Query};

pub struct InsertQuery {
    pub parts: Vec<String>,
    pub args: Vec<Box<dyn Display>>,
}

impl Query for InsertQuery {
    fn updated(&mut self, query: String, args: Option<&mut Vec<impl Display>>) -> &Self {
        self.parts.push(query);
        if let Some(args) = args {
            self.args.append(args);
        }
        self
    }
}

impl InsertQuery {
    pub fn on_duplicate(&mut self) -> &Self {
        self.updated("ON DUPLICATE".to_string(), vec![])
    }

    pub fn update(&mut self, values: impl ColToVal) -> &Self {
        let (query, args) = values.parse();
        self.updated(query, args)
    }

    pub fn ignore(&mut self) -> &Self {
        self.updated("IGNORE".to_string(), vec![])
    }
}

impl mixin::IntoMixin for InsertQuery {}
impl mixin::ValuesMixin for InsertQuery {}
impl mixin::ReturningMixin for InsertQuery {}