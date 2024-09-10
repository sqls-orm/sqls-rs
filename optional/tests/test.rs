use optional::Optional;

#[cfg(feature = "sql")]
#[derive(Optional)]
pub struct User {
    pub id: u8
}

#[cfg(feature = "sql")]
#[tokio::test]
async fn main() {
    let s = User {
        id: 2
    };
    s.id;

    let o = s.optional();
    o.id.unwrap();

    let o = UserOptional {
        id: Some(2)
    };
    o.id.unwrap();

    let s = o.original();
    s.id;
}