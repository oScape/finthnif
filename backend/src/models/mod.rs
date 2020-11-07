use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

pub type FinthnifSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn driver(&self, ctx: &Context<'_>) -> Driver {
        ctx.data_unchecked::<Driver>().clone()
    }
}

#[derive(Clone)]
pub struct Driver {
    firstname: String,
}

#[Object]
impl Driver {
    async fn firstname(&self) -> String {
        self.firstname.clone()
    }
}

impl Default for Driver {
    fn default() -> Self {
        Driver {
            firstname: "Benoit".to_string(),
        }
    }
}
