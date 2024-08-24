use crate::mixin;

struct DeleteQuery {}

impl mixin::FromMixin for DeleteQuery {}
impl mixin::WhereMixin for DeleteQuery {}
impl mixin::ReturningMixin for DeleteQuery {}
