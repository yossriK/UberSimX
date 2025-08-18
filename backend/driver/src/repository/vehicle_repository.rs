use crate::models::Vehicle;
use uuid::Uuid;

pub trait VehicleRepository {
    fn create_vehicle(&self, vehicle: Vehicle) -> Result<Vehicle, String>;
    fn get_vehicle(&self, vehicle_id: Uuid) -> Option<Vehicle>;
    fn update_vehicle(&self, vehicle: Vehicle) -> Result<Vehicle, String>;
    fn delete_vehicle(&self, vehicle_id: Uuid) -> Result<(), String>;
    fn list_vehicles_by_driver(&self, driver_id: Uuid) -> Vec<Vehicle>;
}