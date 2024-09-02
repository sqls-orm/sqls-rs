use builder as sql;
use builder::Model;

mod models;

// #[tokio::test]
// async fn select_default() {
//     let (query, args) = sql::select()
//         .from(models::User::table())
//         .build();
//     println!("{} {:?}", query, args);
// }

#[tokio::test]
async fn select_distinct_count() {
    let (query, args) = sql::select(vec![
        models::User::username().distinct(),
        models::User::password().count().as_("total"),
    ])
        .from(models::User::table())
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn select_specific() {
    let (query, args) = sql::select(vec![
        models::User::username(),
        models::User::password(),
    ])
        .from(models::User::table())
        .build();
    println!("{} {:?}", query, args);
}

#[tokio::test]
async fn select_alias() {
    let (query, args) = sql::select(vec![
        models::User::username().as_("username_alias"),
        models::User::password().as_("password_alias"),
    ])
        .from(models::User::table())
        .build();
    println!("{} {:?}", query, args);
}

// #[tokio::test]
// async fn select_order() {
//     let (query, args) = sql::select()
//         .from(models::User::table())
//         .order_by(vec![
//             models::User::id(),
//             models::User::username().desc(),
//             models::User::password().asc(),
//         ])
//         .build();
//     println!("{} {:?}", query, args);
// }

// #[tokio::test]
// async fn select_pagination() {
//     let (query, args) = sql::select()
//         .from(models::User::table())
//         .limit(100u8)
//         .offset(0u8)
//         .build();
//     println!("{} {:?}", query, args);
// }

// #[tokio::test]
// async fn select_raw_where() {
//     let (query, args) = sql::select()
//         .from(models::User::table())
//         .where_((
//             models::App::id().eq(1)
//                 .and(models::App::name().eq("Telegram"))
//         ).or(
//             models::App::id().eq(2)
//                 .and(models::App::name().eq("Google"))
//         ))
//         .build();
//     println!("{} {:?}", query, args);
// }

// #[tokio::test]
// async fn select_join() {
//     let (query, args) = sql::select()
//         .from(models::User::table())
//         .join(models::App::table())
//         .on((
//             models::User::id().is(models::App::user_id())
//         ).and(
//             models::App::user_id().is(models::User::id())
//         ))
//         .build();
//     println!("{} {:?}", query, args);
// }

#[tokio::test]
async fn select_extra() {
    let (query, args) = sql::select(vec![
        models::User::username().as_("username"),
        models::User::password().as_("password"),
    ])
        .from(models::User::table())
        .join(models::App::table())
        .on(
            models::User::id().is(models::App::user_id())
        )
        .where_((
            models::App::id().eq(1)
                .and(models::App::name().eq("Telegram"))
        ).or(
            models::App::id().eq(2)
                .and(models::App::name().eq("Google"))
        ))
        .order_by(vec![
            models::User::id(),
            models::User::username().desc(),
            models::User::password().asc(),
        ])
        .limit(10u8)
        .offset(0u8)
        .build();
    println!("{} {:?}", query, args);
}