// use builder as sql;
//
// mod models;
//
// // #[tokio::test]
// // async fn insert_default() {
// //     let (query, args) = sql::insert()
// //         .into_("user")
// //         .values(vec![
// //             "username",
// //             "password",
// //         ])
// //         .build();
// //     println!("{} {:?}", query, args);
// // }
// //
// // #[tokio::test]
// // async fn insert_update() {
// //     let (query, args) = sql::insert()
// //         .into_("user")
// //         .values(vec![
// //             "username",
// //             "password",
// //         ]).on_duplicate()
// //         .update(vec![
// //             "password = \"newpass\"",
// //         ])
// //         .build();
// //     println!("{} {:?}", query, args);
// // }
// //
// // #[tokio::test]
// // async fn insert_ignore() {
// //     let (query, args) = sql::insert()
// //         .ignore()
// //         .into_("user")
// //         .values(vec![
// //             "username",
// //             "password",
// //         ])
// //         .build();
// //     println!("{} {:?}", query, args);
// // }
// //
// // #[tokio::test]
// // async fn insert_returning_all() {
// //     let (query, args) = sql::insert()
// //         .into_("user")
// //         .values(vec![
// //             "username",
// //             "password",
// //         ])
// //         .returning()
// //         .build();
// //     println!("{} {:?}", query, args);
// // }
// //
// // #[tokio::test]
// // async fn insert_returning_specific() {
// //     let (query, args) = sql::insert()
// //         .into_("user")
// //         .values(vec![
// //             "username",
// //             "password",
// //         ])
// //         .returning(vec![
// //             "username",
// //             "password",
// //         ])
// //         .build();
// //     println!("{} {:?}", query, args);
// // }