use sql;

#[tokio::test]
async fn select_default() {
    let (query, args) = sql::select().from("user").build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn select_specific() {
    let (query, args) = sql::select(vec![
        models::User::username().as_("username"),
        models::User::password().as_("password"),
    ]).from("user").build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn select_order() {
    let (query, args) = sql::select()
        .from("user")
        .order_by(vec![
            "id",
            "username DESC",
            "password ASC",
        ])
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn select_pagination() {
    let (query, args) = sql::select()
        .from("user")
        .limit(100u8)
        .offset(0u8)
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn select_raw_where() {
    let (query, args) = sql::select()
        .from("user")
        .where_(r#"
            (app.id = 1 AND app.name = \"Telegram\")
            OR
            (app.id = 2 AND app.name = \"Google\")
        "#)
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn select_join() {
    let (query, args) = sql::select()
        .from("user")
        .join("app")
        .on("user.id = app.user_id")
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn select_extra() {
    let (query, args) = sql::select(vec![
        "username",
        "password",
    ]).from("user")
        .join("app")
        .on("user.id = app.user_id")
        .where_(r#"
            (app.id = 1 AND app.name = \"Telegram\")
            OR
            (app.id = 2 AND app.name = \"Google\")
        "#)
        .order_by(vec![
            "id",
            "username DESC",
            "password ASC",
        ])
        .limit(10u8)
        .offset(0u8)
        .build();
    println!("{} {:?}", query, args);
}