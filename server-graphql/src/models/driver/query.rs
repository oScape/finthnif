use async_graphql::{Context, Object};
use finthnif::Finthnif;

use super::Driver;

pub struct Query;

#[Object]
impl Query {
    async fn get_drivers(&self, ctx: &Context<'_>) -> Vec<Driver> {
        ctx.data_unchecked::<Finthnif>()
            .get_drivers()
            .iter()
            .map(|driver| driver.clone().into())
            .collect()
    }
}
