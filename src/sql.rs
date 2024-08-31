use std::any::Any;
use std::fmt::Display;
use std::mem;
use std::sync::{Arc, Mutex};

use crate::types::Column;

pub struct Query {
    pub parts: Arc<Mutex<Vec<String>>>,
    pub args: Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>>,
}

impl Query {
    pub fn new(parts: Vec<String>, args: Vec<Box<dyn Any + Send + Sync>>) -> Self {
        Self {
            parts: Arc::new(Mutex::new(parts)),
            args: Arc::new(Mutex::new(args)),
        }
    }
}

impl Query {
    pub fn from<T>(&self, table: T) -> &Self
    where
        T: Display + 'static,
    {
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("FROM {table}"));
        self
    }

    pub fn into_<T>(&self, table: T) -> &Self
    where
        T: Display + 'static,
    {
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("INTO {table}"));
        self
    }

    pub fn join<T>(&self, table: T) -> &Self
    where
        T: Display + 'static,
    {
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("JOIN {table}"));
        self
    }

    pub fn on<T>(&self, on: T) -> &Self where T: Display + 'static {
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("ON {on}"));
        self
    }

    pub fn limit<T>(&self, limit: T) -> &Self
    where
        T: Into<u128> + Send + Sync + 'static,
    {
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("LIMIT %s"));
        let mut args = self.args.lock().unwrap();
        args.push(Box::new(limit));
        self
    }

    pub fn offset<T>(&self, offset: T) -> &Self
    where
        T: Into<u128> + Send + Sync + 'static,
    {
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("OFFSET %s"));
        let mut args = self.args.lock().unwrap();
        args.push(Box::new(offset));
        self
    }

    pub fn order_by<T>(&self, columns: Vec<T>) -> &Self
    where
        T: Display + 'static,
    {
        let mut part = Vec::<String>::default();
        for col in columns {
            part.push(format!("{col}"));
        }
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("ORDER BY {}", part.join(", ")));
        self
    }

    pub fn returning(&self, columns: Vec<Column>) -> &Self {
        let mut part = Vec::<String>::default();
        for col in columns {
            part.push(col.val.lock().unwrap().to_string());
        }
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("RETURNING {}", part.join(", ")));
        self
    }

    pub fn values(&self, values: Vec<&Column>) -> &Self {
        let mut parts = self.parts.lock().unwrap();
        let mut part = Vec::<String>::default();
        let mut args = self.args.lock().unwrap();
        for col in values {
            part.push(col.val.lock().unwrap().to_string().replace(" = %s", ""));
            args.append(&mut col.args.lock().unwrap());
        }
        parts.push(format!("VALUES ({})", part.join(", ")));
        self
    }

    pub fn where_(&self, conditions: &Column) -> &Self {
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("WHERE {}", conditions.val.lock().unwrap()));
        let mut args = self.args.lock().unwrap();
        args.append(&mut conditions.args.lock().unwrap());
        self
    }

    pub fn on_duplicate(&self) -> &Self {
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("ON DUPLICATE"));
        self
    }

    pub fn update(&self, values: Vec<&Column>) -> &Self {
        let mut parts = self.parts.lock().unwrap();
        let mut part = Vec::<String>::default();
        let mut args = self.args.lock().unwrap();
        for col in values {
            part.push(col.val.lock().unwrap().to_string());
            args.append(&mut col.args.lock().unwrap());
        }
        parts.push(format!("UPDATE {}", part.join(", ")));
        self
    }

    pub fn set(&self, values: Vec<&Column>) -> &Self {
        let mut parts = self.parts.lock().unwrap();
        let mut part = Vec::<String>::default();
        let mut args = self.args.lock().unwrap();
        for col in values {
            part.push(col.val.lock().unwrap().to_string());
            args.append(&mut col.args.lock().unwrap());
        }
        parts.push(format!("SET {}", part.join(", ")));
        self
    }

    pub fn ignore(&self) -> &Self {
        let mut parts = self.parts.lock().unwrap();
        parts.push(format!("IGNORE"));
        self
    }

    pub fn build(&self) -> (String, Vec<Box<dyn Any + Send + Sync>>)
    {
        let mut query = self.parts.lock().unwrap().join(" ");
        query.push(';');
        let args = mem::take(&mut *self.args.lock().unwrap());
        (query, args)
    }
}