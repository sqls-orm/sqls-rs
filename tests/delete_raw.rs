use sql;

#[tokio::test]
async fn delete_default() {
    let (query, args) = sql::delete()
        .from("user")
        .build();
    println!("{} {:?}", query, args);
}