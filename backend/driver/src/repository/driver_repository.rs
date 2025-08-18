use uuid::Uuid;
use anyhow::Error;

use crate::models::Driver;

pub trait DriverRepository {
    fn create_driver(&self, driver: &Driver)  -> Result<Driver, Error>;
    fn get_driver(&self, id: Uuid) -> anyhow::Result<Option<Driver>>;
    fn list_drivers(&self) -> anyhow::Result<Vec<Driver>>;
    fn update_driver(&self, driver: &Driver) -> anyhow::Result<()>;
    fn delete_driver(&self, id: Uuid) -> Result<(), Error>;
}
