pub mod models {
    use std::fmt::{Display, Formatter};
    use sql::types::Column;

    #[derive(Debug)]
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

    #[derive(Debug)]
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

    pub static User: UserModel = UserModel {
        id: Column { val: "id".to_string() },
        username: Column { val: "username".to_string() },
        password: Column { val: "password".to_string() },
    };

    pub static App: AppModel = AppModel {
        id: Column { val: "id".to_string() },
        name: Column { val: "name".to_string() },
        user_id: Column { val: "user_id".to_string() },
    };
}