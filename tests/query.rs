use sql::models;
use sql;

#[tokio::main]
async fn main() {
    let _ = sql::insert().into_(models::User).values(vec![
        *models::User::username().eq("username").eq(models::App::id()),
        *models::User::password().eq("password"),
    ]).on_duplicate().update(vec![
        *models::User::password().eq("newpass"),
    ]).returning(vec![
        models::User::username(),
        models::User::password(),
    ]);

    let _ = sql::select(vec![
        *models::User::username().as_("username"),
        *models::User::password().as_("password"),
    ]).from(models::User).join(models::App).on(
        *models::User::id().eq(models::App::user_id())
    ).where_(*(
        models::App::id().eq(1)
            .and(models::App::name().eq("Telegram"))
    ).or(
        models::App::id().eq(2)
            .and(models::App::name().eq("Google"))
    )).order_by(vec![
        *models::User::id(),
        *models::User::username().desc(),
        *models::User::password().asc(),
    ]).limit(10u8).offset(0u8);
}