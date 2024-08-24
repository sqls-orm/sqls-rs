use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait ReturningMixin: Query {
    fn returning(&mut self, columns: Vec<impl Display>) -> &Self {
        let query = columns.iter().map(|c| format!("{c}")).collect::<Vec<String>>().join(", ");
        let args = None;
        self.updated(query, args)
    }
}