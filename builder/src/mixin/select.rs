pub trait SelectContext {
    fn select<'ctx>(
        &self,
        ctx: Option<&gql::Context>,
    );
}

impl SelectContext for Query {
    fn select<'ctx>(&self, ctx: Option<&gql::Context>) {
        todo!()
    }
}

pub trait SelectDefault {
    fn select<'ctx>(
        &self,
    );
}

impl SelectDefault for Query {
    fn select<'ctx>(&self) {
        todo!()
    }
}