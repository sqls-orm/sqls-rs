use builder as sql;
use builder::Model;

mod models;

#[tokio::test]
async fn update_default() {
    let (query, args) = sql::update(models::User::table())
        .set(vec![
            models::User::username().eq("username"),
            models::User::password().eq("password"),
        ])
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn update_where() {
    let (query, args) = sql::update(models::User::table())
        .set(vec![
            models::User::username().eq("username"),
            models::User::password().eq("password"),
        ])
        .where_(
            models::User::password().eq("newpass")
        )
        .build();
    println!("{} {:?}", query, args);
}

// #[tokio::test]
// async fn update_returning() {
//     let (query, args) = sql::update(models::User::table())
//         .set(vec![
//             models::User::username().eq("username"),
//             models::User::password().eq("password"),
//         ])
//         .returning()
//         .build();
//     println!("{} {:?}", query, args);
// }