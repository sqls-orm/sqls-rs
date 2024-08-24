use crate::queries::_traits::{ColToVal, Query};

pub trait WhereMixin: Query {
    fn where_(&mut self, conditions: impl ColToVal) -> &Self {
        let (query, args) = conditions.parse();
        self.updated(query, Some(args))
    }
}