use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait OrderByMixin: Query {
    fn order_by(&mut self, columns: Vec<impl Display>) -> &Self {
        let query = format!("ORDER BY {}", columns.join(", "));
        let args = None;
        self.updated(query, args)
    }
}