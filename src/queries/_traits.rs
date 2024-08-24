use std::collections::HashMap;
use std::fmt::Display;

use crate::types::Column;

pub trait Query {
    fn updated(&mut self, query: String, args: Option<&mut Vec<impl Display>>) -> &Self;
}

pub trait ColToVal {
    fn parse(&mut self) -> (String, &mut Vec<Box<impl Display>>);
}

impl ColToVal for Column {
    fn parse(&mut self) -> (String, &mut Vec<Box<impl Display>>) {
        (format!("{}", self.val), &mut self.args)
    }
}

impl ColToVal for Vec<Column> {
    fn parse(&mut self) -> (String, &mut Vec<Box<impl Display>>) {
        let mut query = String::default();
        let mut args = vec![];
        for col in self {
            query.push_str(col.val.as_str());
            args.append(&mut col.args)
        }
        (query, &mut args)
    }
}

impl ColToVal for HashMap<Column, dyn Display> {
    fn parse(&mut self) -> (String, &mut Vec<Box<impl Display>>) {
        let mut query = String::default();
        let mut args = vec![];
        for (col, val) in self.iter() {
            let (query_p, args_p) = col.eq(val).parse();
            query.push_str(query_p.as_str());
            args.append(args_p);
        }
        (query, &args)
    }
}