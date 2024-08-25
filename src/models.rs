use std::fmt::{Display, Formatter};

use crate::types::Column;

pub struct User {
    pub id: u8,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn id() -> Column {
        Column::from("id".to_string())
    }
    pub fn username() -> Column {
        Column::from("username".to_string())
    }
    pub fn password() -> Column {
        Column::from("password".to_string())
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("user")
    }
}

pub struct App {
    pub id: Column,
    pub name: Column,
    pub user_id: Column,
}

impl App {
    pub fn id() -> Column {
        Column::from("id".to_string())
    }
    pub fn name() -> Column {
        Column::from("name".to_string())
    }
    pub fn user_id() -> Column {
        Column::from("user_id".to_string())
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("app")
    }
}