use crate::types::Column;

pub struct User {
    pub id: u8,
    pub username: String,
    pub password: String,
}

pub trait Model {
    fn table() -> &'static str;
}

impl User {
    pub fn id() -> Column {
        Column::new("id")
    }
    pub fn username() -> Column {
        Column::new("username")
    }
    pub fn password() -> Column {
        Column::new("password")
    }
}

impl Model for User {
    fn table() -> &'static str {
        "user"
    }
}

pub struct App {
    pub id: Column,
    pub name: Column,
    pub user_id: Column,
}

impl App {
    pub fn id() -> Column {
        Column::new("id")
    }
    pub fn name() -> Column {
        Column::new("name")
    }
    pub fn user_id() -> Column {
        Column::new("user_id")
    }
}

impl Model for App {
    fn table() -> &'static str {
        "app"
    }
}