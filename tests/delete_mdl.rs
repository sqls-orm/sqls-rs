use sql::models::Model;
use sql::models;
use sql;

#[tokio::test]
async fn delete_default() {
    let (query, args) = sql::delete()
        .from(models::User::table())
        .build();
    println!("{} {:?}", query, args);
}