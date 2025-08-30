use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Driver;

#[async_trait]
pub trait DriverRepository {
    async fn create_driver(&self, driver: &Driver) -> Result<(), Error>;
    async fn get_driver(&self, id: Uuid) -> anyhow::Result<Option<Driver>>;
    async fn list_drivers(&self) -> anyhow::Result<Vec<Driver>>;
    async fn update_driver(&self, driver: &Driver) -> anyhow::Result<()>;
    async fn delete_driver(&self, id: Uuid) -> Result<(), Error>;
}

// Struct that holds the database pool
#[derive(Clone)]
pub struct PgDriverRepository {
    pub pool: Arc<PgPool>,
}

impl PgDriverRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DriverRepository for PgDriverRepository {
    async fn create_driver(&self, driver: &Driver) -> Result<(), Error> {
      let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO drivers (id, name, license_number, rating, car_id)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id) // $1 → id
    .bind(&driver.name) // $2 → name
    .bind(driver.license_number.as_ref()) // $3 → license_number
    .bind(driver.rating) // $4 → rating
    .bind(driver.car_id) // $5 → car_id
    .execute(self.pool.as_ref())
    .await?;
    
    
    Ok(())
        
    }

    async fn get_driver(&self, id: Uuid) -> anyhow::Result<Option<Driver>> {
      todo!()
    }

    async fn list_drivers(&self) -> anyhow::Result<Vec<Driver>> {
        // let recs = sqlx::query_as!(
        //     Driver,
        //     r#"
        //     SELECT id, name, rating
        //     FROM drivers
        //     "#
        // )
        // .fetch_all(self.pool.as_ref())
        // .await?;
        // Ok(recs)
                todo!()

    }

    async fn update_driver(&self, driver: &Driver) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete_driver(&self, id: Uuid) -> Result<(), Error> {
        todo!()
    }
}
