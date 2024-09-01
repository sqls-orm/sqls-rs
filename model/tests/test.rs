use model::Model;

#[derive(Model)]
struct User {
    pub id: u8,
}

#[tokio::test]
async fn main() {
    let col = Column::new("id");

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

    User::id().and(User::id());
    User::id().or(User::id());
}