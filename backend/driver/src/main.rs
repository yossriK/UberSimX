mod models;

pub mod repository {
    pub mod driver_repository;
    pub mod vehicle_repository;
    // DriverLocation usually not persisted in a database for high frequency updates. will be in memrory or cache
    
    // DriverAvailabilityEvent don't need to be stored in a database, they are transient messages, that exist as events 
    // in the messaging system

    // DriverStatus could be peristed, but for simulation in memroy is fine.  
}

pub mod api {
    pub mod driver;
    pub mod router;
}

fn main() {
    println!("Hello, world!");
}
