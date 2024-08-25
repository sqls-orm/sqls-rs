use sql;
use sql::mixin::*;
use sql::models;

#[test]
async fn main() {
    let _ = sql::insert().into_(models::User).values(vec![
        models::User.username.eq("username"),
        models::User.password.eq("password"),
    ]).on_duplicate().update(vec![
        models::User.password.eq("newpass"),
    ]).returning(vec![
        &models::User.username,
        &models::User.password,
    ]);
}