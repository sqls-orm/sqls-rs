use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait JoinMixin: Query {
    fn join(&mut self, table: impl Display) -> &Self {
        let query = format!("JOIN {table}");
        let args = None;
        self.updated(query, args)
    }

    fn on(&mut self, on: impl Display) -> &Self {
        let query = format!("ON {on}");
        let args = None;
        self.updated(query, args)
    }
}