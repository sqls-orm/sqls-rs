use std::fmt::Display;

use crate::queries::_traits::Query;

pub trait JoinMixin: Query {
    fn join(&mut self, table: impl Display) -> &Self {
        self.updated(format!("JOIN {table}"), None::<&mut Vec<_>>)
    }

    fn on(&mut self, on: impl Display) -> &Self {
        self.updated(format!("ON {on}"), None::<&mut Vec<_>>)
    }
}