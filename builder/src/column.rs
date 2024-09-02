use std::any::Any;
use std::fmt::{Display, Formatter, Result};
use std::sync::{Arc, Mutex};

pub struct Column {
    pub val: Arc<Mutex<String>>,
    pub args: Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>>,
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.val.lock().unwrap().as_str())
    }
}

impl Column {
    pub fn new(val: &str) -> Self {
        let val = Arc::new(Mutex::new(val.to_string()));
        let args = Arc::new(Mutex::new(Vec::new()));

        Self { val, args }
    }
}

impl Column {
    pub fn asc(self) -> Self {
        *self.val.lock().unwrap() = format!("{} ASC", self.val.lock().unwrap());
        self
    }

    pub fn desc(self) -> Self {
        *self.val.lock().unwrap() = format!("{} DESC", self.val.lock().unwrap());
        self
    }
}

impl Column {
    pub fn count(self) -> Self {
        *self.val.lock().unwrap() = format!("COUNT({})", self.val.lock().unwrap());
        self
    }

    pub fn distinct(self) -> Self {
        *self.val.lock().unwrap() = format!("DISTINCT {}", self.val.lock().unwrap());
        self
    }
}

impl Column {
    pub fn as_<T>(self, alias: T) -> Self
    where
        T: Display + 'static,
    {
        *self.val.lock().unwrap() = format!("{} AS {alias}", self.val.lock().unwrap());
        self
    }
}

impl Column {
    fn logical(&self, operator: &str, other: &Self) -> &Self {
        let mut val = self.val.lock().unwrap();
        *val = format!("({val} {operator} {other})");
        let mut args = self.args.lock().unwrap();
        args.append(&mut other.args.lock().unwrap());

        self
    }

    pub fn or(&self, other: &Self) -> &Self {
        self.logical("OR", other)
    }

    pub fn and(&self, other: &Self) -> &Self {
        self.logical("AND", other)
    }
}

impl Column {
    pub fn is(self, other: Column) -> Self {
        *self.val.lock().unwrap() = format!("{} = {}", self.val.lock().unwrap(), other.val.lock().unwrap());
        self
    }
}

impl Column {
    fn compare<T>(&self, sign: &str, other: T) -> &Self
    where
        T: Any + Send + Sync,
    {
        let mut val = self.val.lock().unwrap();
        *val = format!("{val} {sign} %s");
        let mut args = self.args.lock().unwrap();
        args.push(Box::new(other));
        self
    }

    pub fn eq<T>(&self, other: T) -> &Self
    where
        T: Any + Send + Sync,
    {
        self.compare("=", other)
    }

    pub fn ne<T>(&self, other: T) -> &Self
    where
        T: Any + Send + Sync,
    {
        self.compare("!=", other)
    }

    pub fn gt<T>(&self, other: T) -> &Self
    where
        T: Any + Send + Sync,
    {
        self.compare(">", other)
    }

    pub fn ge<T>(&self, other: T) -> &Self
    where
        T: Any + Send + Sync,
    {
        self.compare(">=", other)
    }

    pub fn lt<T>(&self, other: T) -> &Self
    where
        T: Any + Send + Sync,
    {
        self.compare("<", other)
    }

    pub fn le<T>(&self, other: T) -> &Self
    where
        T: Any + Send + Sync,
    {
        self.compare("<=", other)
    }
}