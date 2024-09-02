pub use vendor::{
    insert,
    upsert,
    select,
    update,
    delete,
};

pub use column::Column;

mod query;
mod column;
mod vendor;

pub trait Model {
    fn table() -> &'static str;
}