// use builder as sql;
//
// mod models;
//
// #[tokio::test]
// async fn delete_default() {
//     let (query, args) = sql::delete()
//         .from("user")
//         .build();
//     println!("{} {:?}", query, args);
// }