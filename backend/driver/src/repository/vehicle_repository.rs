use std::sync::Arc;

use crate::models::Vehicle;
use sqlx::PgPool;
use uuid::Uuid;

pub trait VehicleRepository {
    fn create_vehicle(&self, vehicle: Vehicle) -> Result<Vehicle, String>;
    fn get_vehicle(&self, vehicle_id: Uuid) -> Option<Vehicle>;
    fn update_vehicle(&self, vehicle: Vehicle) -> Result<Vehicle, String>;
    fn delete_vehicle(&self, vehicle_id: Uuid) -> Result<(), String>;
    fn list_vehicles_by_driver(&self, driver_id: Uuid) -> Vec<Vehicle>;
}

// Struct that holds the database pool
#[derive(Clone)]
pub struct PgVehicleRepository {
    pub pool: Arc<PgPool>,
}

impl PgVehicleRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}


impl VehicleRepository for PgVehicleRepository {
    fn create_vehicle(&self, _vehicle: Vehicle) -> Result<Vehicle, String> {
        todo!()
    }

    fn get_vehicle(&self, _vehicle_id: Uuid) -> Option<Vehicle> {
        todo!()
    }

    fn update_vehicle(&self, _vehicle: Vehicle) -> Result<Vehicle, String> {
        todo!()
    }

    fn delete_vehicle(&self, _vehicle_id: Uuid) -> Result<(), String> {
        todo!()
    }

    fn list_vehicles_by_driver(&self, _driver_id: Uuid) -> Vec<Vehicle> {
        todo!()
    }
}