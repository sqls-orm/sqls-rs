use autogen::models;
use sql;

mod autogen;

#[tokio::test]
async fn main() {
    let _ = sql::insert().into_(&models::User).values(vec![
        models::User.username.eq("username"),
        models::User.password.eq("password"),
    ]).on_duplicate().update(sql::map!{
        &models::User.password => "newpass",
    }).returning(vec![
        models::User.username,
        models::User.password,
    ]);

    let _ = sql::insert().into_(&models::User).values(vec![
        models::User.username.eq("username"),
        models::User.password.eq("password"),
    ]).on_duplicate().update(sql::map!{
        &models::User.password => "newpass",
    }).returning("*");
}