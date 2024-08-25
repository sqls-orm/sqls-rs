use std::fmt::{Display, Formatter};
use std::sync::Arc;
use lazy_static;

use crate::types::Column;
use crate::utils;

struct UserModel {
    pub id: Column,
    pub username: Column,
    pub password: Column,
}

impl Display for UserModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("user")
    }
}

struct AppModel {
    pub id: Column,
    pub name: Column,
    pub user_id: Column,
}

impl Display for AppModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("app")
    }
}

lazy_static::lazy_static! {
    pub static ref User: UserModel = UserModel {
        id: Column { val: "id".to_string(), args: utils::args() },
        username: Column { val: "username".to_string(), args: utils::args() },
        password: Column { val: "password".to_string(), args: utils::args() },
    };

    pub static ref App: AppModel = AppModel {
        id: Column { val: "id".to_string(), args: utils::args() },
        name: Column { val: "name".to_string(), args: utils::args() },
        user_id: Column { val: "user_id".to_string(), args: utils::args() },
    };
}