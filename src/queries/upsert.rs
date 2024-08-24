use crate::mixin;

struct InsertQuery {}

impl mixin::IntoMixin for InsertQuery {}
impl mixin::ValuesMixin for InsertQuery {}
impl mixin::ReturningMixin for InsertQuery {}
