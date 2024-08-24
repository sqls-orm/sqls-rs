#[derive(Debug)]
struct Column {
    val: &'static str,
}

impl Column {
    pub fn alias(&self, alias: String) -> Column {
        Column { val: format!("{} AS {}", self.val, alias).as_str() }
    }
}

impl std::ops::BitAnd for Column {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl std::ops::BitOr for Column {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl Column {
    pub fn eq(&self, other: Column) -> Column {
        Column { val: format!("{} = {}", self.val, other.val).as_str() }
    }

    pub fn ne(&self, other: Column) -> Column {
        Column { val: format!("{} != {}", self.val, other.val).as_str() }
    }

    pub fn gt(&self, other: Column) -> Column {
        Column { val: format!("{} > {}", self.val, other.val).as_str() }
    }

    pub fn ge(&self, other: Column) -> Column {
        Column { val: format!("{} >= {}", self.val, other.val).as_str() }
    }

    pub fn lt(&self, other: Column) -> Column {
        Column { val: format!("{} < {}", self.val, other.val).as_str() }
    }

    pub fn le(&self, other: Column) -> Column {
        Column { val: format!("{} <= {}", self.val, other.val).as_str() }
    }

    pub fn or(&self, other: Column) -> Column {
        Column { val: format!("{} OR {}", self.val, other.val).as_str() }
    }

    pub fn and(&self, other: Column) -> Column {
        Column { val: format!("{} AND {}", self.val, other.val).as_str() }
    }
}

#[derive(Debug)]
struct UserModel {
    pub __tablename__: &'static str,
    pub id: Column,
    pub username: Column,
    pub password: Column,
}

#[derive(Debug)]
struct AppModel {
    pub __tablename__: &'static str,
    pub id: Column,
    pub name: Column,
    pub user_id: Column,
}

pub static User: UserModel = UserModel {
    __tablename__: "user",
    id: Column { val: "id" },
    username: Column { val: "username" },
    password: Column { val: "password" },
};

pub static App: AppModel = AppModel {
    __tablename__: "app",
    id: Column { val: "id" },
    name: Column { val: "name" },
    user_id: Column { val: "user_id" },
};