use std::fmt::Display;
use crate::types::Column;

pub fn args() -> Vec<Box<dyn Display>> {
    vec![]
}

pub fn join<T>(v: Vec<T>) -> String
where
    T: Display,
{
    v.iter().map(|c| format!("{c}")).collect::<Vec<String>>().join(", ")
}

pub fn merge<T>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<Box<dyn Display + 'static>>
where
    T: Display,
{
    let mut v = Vec::<Box<dyn Display + 'static>>::default();
    for e in v1.iter() {
        v.push(Box::new(format!("{e}")));
    }
    for e in v2 {
        v.push(Box::new(format!("{e}")));
    }
    v
}

pub fn parse(v: Vec<Column>) -> (String, Vec<Box<dyn Display>>) {
    let mut query = String::default();
    let mut args = vec![];
    for col in v {
        query.push_str(col.val.as_str());
        for e in col.args {
            args.push(e);
        }
    }
    (query, args)
}