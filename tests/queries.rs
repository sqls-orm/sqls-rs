

macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}

#[tokio::test]
async fn main() {
    let query = sqlx::insert().into(models::UserModel).values(map!{
        models::UserModel
    })
}