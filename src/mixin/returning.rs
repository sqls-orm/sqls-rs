use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait ReturningMixin: Query {
    fn returning(&mut self, columns: Vec<impl Display>) -> &Self {
        self.updated(columns.iter().map(|c| format!("{c}")).collect::<Vec<String>>().join(", "), None::<&mut Vec<_>>)
    }
}