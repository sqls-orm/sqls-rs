use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait OrderByMixin: Query {
    fn order_by(&mut self, columns: Vec<impl Display>) -> &Self {
        self.updated(format!("ORDER BY {}", columns.join(", ")), None::<&mut Vec<_>>)
    }
}