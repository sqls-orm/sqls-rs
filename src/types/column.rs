use std::fmt::{Display, Formatter, Result};

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
        Self { val: format!("{} OR {}", self.val, other), args: [&self.args, &other.args].concat() }
    }

    pub fn and(&self, other: Self) -> Self {
        Self { val: format!("{} AND {}", self.val, other), args: [&self.args, &other.args].concat() }
    }
}

impl Column {
    pub fn eq(&self, other: impl Display) -> Self {
        Self { val: format!("{} = ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn ne(&self, other: impl Display) -> Self {
        Self { val: format!("{} != ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn gt(&self, other: impl Display) -> Self {
        Self { val: format!("{} > ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn ge(&self, other: impl Display) -> Self {
        Self { val: format!("{} >= ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn lt(&self, other: impl Display) -> Self {
        Self { val: format!("{} < ?", self.val), args: vec![Box::new(other)] }
    }

    pub fn le(&self, other: impl Display) -> Self {
        Self { val: format!("{} <= ?", self.val), args: vec![Box::new(other)] }
    }
}