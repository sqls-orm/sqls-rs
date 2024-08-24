use crate::queries::_traits::Query;
use crate::types::Column;

pub trait WhereMixin: Query {
    fn where_(&mut self, conditions: Column) -> &Self {
        self.updated(conditions.val, Some(conditions.args))
    }
}