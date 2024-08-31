use sql::models::Model;
use sql::models;
use sql;

#[tokio::test]
async fn insert_default() {
    let (query, args) = sql::insert()
        .into_(models::User::table())
        .values(vec![
            models::User.username().eq("username"),
            models::User.password().eq("password"),
        ])
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn insert_update() {
    let (query, args) = sql::insert()
        .into_(models::User::table())
        .values(vec![
            models::User.username().eq("username"),
            models::User.password().eq("password"),
        ]).on_duplicate()
        .update(vec![
            models::User.password().eq("newpass"),
        ])
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn insert_ignore() {
    let (query, args) = sql::insert()
        .ignore()
        .into_(models::User::table())
        .values(vec![
            models::User.username().eq("username"),
            models::User.password().eq("password"),
        ])
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn insert_returning_all() {
    let (query, args) = sql::insert()
        .into_(models::User::table())
        .values(vec![
            models::User.username().eq("username"),
            models::User.password().eq("password"),
        ])
        .returning()
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn insert_returning_specific() {
    let (query, args) = sql::insert()
        .into_(models::User::table())
        .values(vec![
            models::User.username().eq("username"),
            models::User.password().eq("password"),
        ])
        .returning(vec![
            models::User::username(),
            models::User::password(),
        ])
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn insert_returning_alias() {
    let (query, args) = sql::insert()
        .into_(models::User::table())
        .values(vec![
            models::User.username().eq("username"),
            models::User.password().eq("password"),
        ])
        .returning(vec![
            models::User::username().as_("username_alias"),
            models::User::password().as_("password_alias"),
        ])
        .build();
    println!("{} {:?}", query, args);
}