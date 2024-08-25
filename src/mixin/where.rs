use crate::queries::_traits::Query;
use crate::types::Column;

pub trait WhereMixin: Query {
    fn where_(&mut self, conditions: Column) -> &Self {
        let query = conditions.val;
        let args = conditions.args;
        self.updated(query, args)
    }
}