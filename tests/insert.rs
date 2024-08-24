use autogen::models;
use sql;

mod autogen;

#[tokio::test]
async fn main() {
    let _ = sql::insert().into_(&models::User).values(sql::map!{
        &models::User.username => "username",
        &models::User.password => "password",
    }).on_duplicate().update(sql::map!{
        &models::User.password => "newpass",
    }).returning();
}