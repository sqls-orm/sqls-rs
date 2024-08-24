use crate::queries::_traits::{ColToVal, Query};

pub trait ValuesMixin: Query {
    fn values(&mut self, values: impl ColToVal) -> &Self {
        let (query, args) = values.parse();
        self.updated(query, Some(args))
    }
}