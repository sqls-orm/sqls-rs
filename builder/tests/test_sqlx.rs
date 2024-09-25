use sqlx::MySql;
use builder as sql;
use builder::Model;
// use model::Model;
use optional::Model;

mod models;

#[derive(Model)]
struct User {
    pub id: u8,
    pub username: String,
    pub password: String,
}

#[tokio::test]
async fn select_distinct_count() {
    dotenv::dotenv().ok();

    sqlx::query_as::<MySql, UserModel>(r#"
         SELECT username
         FROM user
         WHERE id = %s
    "#);

    let (query, args) = sql::select(vec![
        models::User::username().distinct(),
        models::User::password().count().as_("total"),
    ])
        .from(models::User::table())
        .build();
    println!("{} {:?}", query, args);
}