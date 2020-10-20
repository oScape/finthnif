use async_graphql::{Context, Object};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn drivers(&self, _ctx: &Context<'_>) -> Vec<Driver> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Driver {
    first_name: String,
}

#[Object]
impl Driver {
    async fn first_name(&self) -> String {
        self.first_name.clone()
    }
}

impl Default for Driver {
    fn default() -> Self {
        Driver {
            first_name: "Benoit".to_string(),
        }
    }
}
