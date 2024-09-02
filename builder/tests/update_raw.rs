use builder as sql;

mod models;

// #[tokio::test]
// async fn update_default() {
//     let (query, args) = sql::update("user")
//         .set(vec![
//             "username = \"username\"",
//             "password = \"password\"",
//         ])
//         .build();
//     println!("{} {:?}", query, args);
// }

// #[tokio::test]
// async fn update_where() {
//     let (query, args) = sql::update("user")
//         .set(vec![
//             "username = \"username\"",
//             "password = \"password\"",
//         ])
//         .where_(vec![
//             "password = \"newpass\"",
//         ])
//         .build();
//     println!("{} {:?}", query, args);
// }

// #[tokio::test]
// async fn update_returning() {
//     let (query, args) = sql::update("user")
//         .set(vec![
//             "username = \"username\"",
//             "password = \"password\"",
//         ])
//         .returning()
//         .build();
//     println!("{} {:?}", query, args);
// }