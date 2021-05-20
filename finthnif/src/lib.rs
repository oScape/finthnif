pub mod models;

use models::driver::{Driver, DriverModel};

pub struct Finthnif {
    driver: Driver,
}

impl Finthnif {
    pub fn new() -> Self {
        Self {
            driver: DriverModel::new(Driver {
                firstname: "Franck".to_string(),
                lastname: "Chassignol".to_string(),
            }),
        }
    }

    pub fn get_drivers(&self) -> Vec<Driver> {
        [self.driver.clone()].to_vec()
    }
}
