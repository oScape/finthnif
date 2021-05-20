use super::dto::Driver;
// use anyhow::{anyhow, Result as AnyResult};
// use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct DriverModel {
    // pool: Pool<Postgres>,
}

impl DriverModel {
    pub fn new(driver: Driver) -> Driver {
        Driver {
            firstname: driver.firstname,
            lastname: driver.lastname,
        }
    }
    // pub async fn with_connection_string(url: &str) -> AnyResult<Self> {
    //     let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

    //     Ok(Self { pool })
    // }

    // pub async fn create(&self, driver: Driver) -> AnyResult<Driver> {
    //     sqlx::query!(
    //         r#"insert into driver (lastname, firstname) values($1::text, $2::text)"#,
    //         driver.lastname,
    //         driver.firstname,
    //     )
    //     .execute(&self.pool)
    //     .await?;

    //     self.by_lastname(driver.lastname)
    //         .await
    //         .map(Ok)
    //         .ok_or_else(|| anyhow!("driver does not exist"))?
    // }

    // pub async fn by_lastname(&self, lastname: String) -> Option<Driver> {
    //     sqlx::query_as!(
    //         Driver,
    //         r#"select lastname, firstname, created_at, updated_at from driver where lastname = $1::text"#,
    //         lastname,
    //     )
    //     .fetch_one(&self.pool)
    //     .await
    //     .map(Some)
    //     .unwrap_or_default()
    // }
}
