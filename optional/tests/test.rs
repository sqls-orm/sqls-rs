use optional::Model;

#[derive(Model)]
pub struct User {
    pub id: u8
}

#[tokio::test]
async fn main() {
    let s = User {
        id: 2
    };
    s.id;

    let o = s.model();
    o.id.unwrap();

    let o = UserModel {
        id: Some(2)
    };
    o.id.unwrap();

    let s = o.structure();
    s.id;
}