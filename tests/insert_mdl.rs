use sql::models::Model;
use sql::models;
use sql;

#[tokio::test]
async fn insert_default() {
    let (query, args) = sql::insert().into_(models::User::table()).values(vec![
        models::User::username().eq("username"),
        models::User::password().eq("password"),
    ]).on_duplicate().update(vec![
        models::User::password().eq("newpass"),
    ]).returning(vec![
        models::User::username(),
        models::User::password(),
    ]).build();
    println!("{} {:?}", query, args);
}