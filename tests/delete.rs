use autogen::models;
use sql;

mod autogen;

#[tokio::test]
async fn main() {
    let _ = sql::select(vec![
        models::User.username.as_("username"),
        models::User.password.as_("password"),
    ]).from(&models::User).join(&models::App).on(
        models::User.id.eq(&models::App.user_id)
    ).where_(
        (models::App.name.eq("Telegram")
            .or(models::App.name.eq("Google"))
        ).and(models::App.id.eq(1))
    ).order_by(vec![
        models::User.username.desc(),
    ]).limit(10).offset(0);
}