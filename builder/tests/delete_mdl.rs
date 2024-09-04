// use builder as sql;
// use builder::Model;
//
// mod models;
//
// #[tokio::test]
// async fn delete_default() {
//     let (query, args) = sql::delete()
//         .from(models::User::table())
//         .build();
//     println!("{} {:?}", query, args);
// }