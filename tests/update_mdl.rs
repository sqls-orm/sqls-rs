use sql::models::Model;
use sql::models;
use sql;

#[tokio::test]
async fn update_default() {
    let (query, args) = sql::update(models::User::table())
        .set(vec![
            models::User.username().eq("username"),
            models::User.password().eq("password"),
        ])
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn update_where() {
    let (query, args) = sql::update(models::User::table())
        .set(vec![
            models::User.username().eq("username"),
            models::User.password().eq("password"),
        ])
        .where_(vec![
            models::User.password().eq("newpass")
        ])
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn update_returning() {
    let (query, args) = sql::update(models::User::table())
        .set(vec![
            models::User.username().eq("username"),
            models::User.password().eq("password"),
        ])
        .returning()
        .build();
    println!("{} {:?}", query, args);
}