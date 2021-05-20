use async_graphql::SimpleObject;

pub mod query;

#[derive(SimpleObject)]
pub struct Driver {
    pub lastname: String,
    pub firstname: String,
}

impl From<finthnif::models::driver::Driver> for Driver {
    fn from(driver: finthnif::models::driver::Driver) -> Self {
        Driver {
            lastname: driver.lastname,
            firstname: driver.firstname,
        }
    }
}
