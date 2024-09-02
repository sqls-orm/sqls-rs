use model::Model;
use builder::Column;

#[derive(Model)]
struct User {
    pub id: u8,
}

#[tokio::test]
async fn main() {
    Column::new("id");

    format!("{}", User::id());

    User::id().is(User::id());

    User::id().asc();
    User::id().desc();

    User::id().distinct();

    User::id().count();

    User::id().eq("other");
    User::id().ne("other");
    User::id().lt("other");
    User::id().le("other");
    User::id().gt("other");
    User::id().ge("other");

    User::id().eq("other").and(User::id().eq("other"));
    User::id().eq("other").or(User::id().eq("other"));
}