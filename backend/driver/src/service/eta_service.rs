use rand::Rng;

/// Dummy ETA service that returns a random ETA between 5 and 10 minutes.
pub struct EtaService;

pub trait EtaCalculator {
    /// Returns ETA in minutes.
    fn calculate_eta_minutes(&self) -> u32;
}

impl EtaCalculator for EtaService {
    fn calculate_eta_minutes(&self) -> u32 {
        let mut rng = rand::rng();
        rng.random_range(5..=10)
    }
}
