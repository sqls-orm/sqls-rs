// use builder as sql;
// use builder::Model;
// use model::Model;
// use optional::Optional;
//
// mod models;
//
// #[derive(Optional, Model)]
// struct User {
//     pub id: u8,
//     pub username: String,
//     pub password: String,
// }
//
// #[tokio::test]
// async fn select_distinct_count() {
//     dotenv::dotenv().ok();
//
//     sqlx::query_as::<_, UserOptional>(r#"
//          SELECT username
//          FROM user
//          WHERE id = %s
//     "#)
//         .bind(0u8)
//         .fetch_optional()
//         .await
//         .unwrap_or_else(|e| {
//             println!("{}", e.to_string());
//             None
//         });
//
//     let (query, args) = sql::select(vec![
//         models::User::username().distinct(),
//         models::User::password().count().as_("total"),
//     ])
//         .from(models::User::table())
//         .build();
//     println!("{} {:?}", query, args);
// }