use uuid::Uuid;
use anyhow::Error;

use crate::models::Driver;

pub trait DriverRepository {
    async fn create_driver(&self, driver: &Driver)  -> Result<Driver, Error>;
    async fn get_driver(&self, id: Uuid) -> anyhow::Result<Option<Driver>>;
    async fn list_drivers(&self) -> anyhow::Result<Vec<Driver>>;
    async fn update_driver(&self, driver: &Driver) -> anyhow::Result<()>;
    async fn delete_driver(&self, id: Uuid) -> Result<(), Error>;
}
