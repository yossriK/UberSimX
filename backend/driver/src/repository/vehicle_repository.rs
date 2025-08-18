use crate::models::Vehicle;
use uuid::Uuid;
use anyhow::Error;

pub trait VehicleRepository {
    fn create_vehicle(&self, vehicle: Vehicle) -> Result<Vehicle, Error>;
    fn get_vehicle(&self, vehicle_id: Uuid) -> Result<Option<Vehicle>, Error>;
    fn update_vehicle(&self, vehicle: Vehicle) -> Result<Vehicle, Error>;
    fn delete_vehicle(&self, vehicle_id: Uuid) -> Result<(), Error>;
    fn list_vehicles_by_driver(&self, driver_id: Uuid) -> Result<Vec<Vehicle>, Error>;
}