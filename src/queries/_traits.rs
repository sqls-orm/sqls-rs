use std::collections::HashMap;
use std::fmt::Display;

use crate::types::Column;

pub trait Query {
    fn updated<T>(&mut self, query: String, args: Vec<T>) -> &Self
    where
        T: Display + 'static;
}

// pub trait Columns {
//     fn parse<T>(&self) -> (String, Vec<T>)
//     where
//         T: Display;
// }
//
// impl Columns for Vec<Column> {
//     fn parse<T>(&self) -> (String, Vec<T>)
//     where
//         T: Display,
//     {
//         let mut query = String::default();
//         let mut args = Vec::<T>::default();
//         for col in self {
//             query.push_str(col.val.as_str());
//             for e in col.args {
//                 args.push(*e);
//             }
//         }
//         (query, args)
//     }
// }
//
// pub trait ColumnValue {
//     fn parse<T>(&self) -> (String, Vec<T>)
//     where
//         T: Display;
// }
//
// impl ColumnValue for Column {
//     fn parse<T>(&self) -> (String, Vec<T>)
//     where
//         T: Display,
//     {
//         let args = self.args.iter().map(|e| *e).collect();
//         (format!("{}", self.val), args)
//     }
// }
//
// impl ColumnValue for Vec<Column> {
//     fn parse<T>(&self) -> (String, Vec<T>)
//     where
//         T: Display,
//     {
//         let mut query = String::default();
//         let mut args = Vec::<T>::default();
//         for col in self {
//             query.push_str(col.val.as_str());
//             for e in col.args {
//                 args.push(*e);
//             }
//         }
//         (query, args)
//     }
// }
//
// impl<V: Display> ColumnValue for HashMap<Column, V> {
//     fn parse<T>(&self) -> (String, Vec<T>)
//     where
//         T: Display,
//     {
//         let mut query = String::default();
//         let mut args = Vec::<T>::default();
//         for (col, val) in self {
//             let (query_p, args_p) = col.eq(val).parse::<T>();
//             query.push_str(query_p.as_str());
//             for e in args_p {
//                 args.push(e);
//             }
//         }
//         (query, args)
//     }
// }