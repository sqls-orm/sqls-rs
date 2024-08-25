use std::fmt::{Display, Formatter, Result};

use crate::utils;

pub struct Column {
    pub val: String,
    pub args: Vec<Box<dyn Display>>,
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.val.as_str())
    }
}

impl Column {
    pub fn asc(&self) -> Self {
        Self { val: format!("{} ASC", self.val), args: vec![] }
    }

    pub fn desc(&self) -> Self {
        Self { val: format!("{} DESC", self.val), args: vec![] }
    }
}

impl Column {
    pub fn count(&self) -> Self {
        Self { val: format!("COUNT({})", self.val), args: vec![] }
    }

    pub fn distinct(&self) -> Self {
        Self { val: format!("DISTINCT {}", self.val), args: vec![] }
    }
}

impl Column {
    pub fn as_(&self, alias: impl Display) -> Self {
        Self { val: format!("{} AS {}", self.val, alias), args: vec![] }
    }
}

impl Column {
    pub fn or(&self, other: Self) -> Self {
        let val = format!("{} OR {}", self.val, other);
        let args = utils::merge(&self.args, &other.args);
        Self { val, args }
    }

    pub fn and(&self, other: Self) -> Self {
        let val = format!("{} AND {}", self.val, other);
        let args = utils::merge(&self.args, &other.args);
        Self { val, args }
    }
}

impl Column {
    pub fn eq<T>(&self, other: T) -> Self
    where
        T: Display + 'static,
    {
        Self { val: format!("{} = ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn ne<T>(&self, other: T) -> Self
    where
        T: Display + 'static,
    {
        Self { val: format!("{} != ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn gt<T>(&self, other: T) -> Self
    where
        T: Display + 'static,
    {
        Self { val: format!("{} > ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn ge<T>(&self, other: T) -> Self
    where
        T: Display + 'static,
    {
        Self { val: format!("{} >= ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn lt<T>(&self, other: T) -> Self
    where
        T: Display + 'static,
    {
        Self { val: format!("{} < ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn le<T>(&self, other: T) -> Self
    where
        T: Display + 'static,
    {
        Self { val: format!("{} <= ?", self.val), args: vec![Box::new(other)] }
    }
}