use crate::queries::_traits::Query;
use crate::types::Column;
use crate::utils;

pub trait ValuesMixin: Query {
    fn values(&mut self, values: Vec<Column>) -> &mut Self
    {
        let (query, args) = utils::parse(values);
        self.updated(query, args)
    }
}