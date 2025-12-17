use serde::{Deserialize, Serialize};
use uuid::Uuid;
// disclaimer I chatted with cahtgpt on the best way to create these structs
// I learnt abut the envelope pattern from it and how to flatten structs using serde

/// Server → Client message types (what driver service sends).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WSMsgType {
    DriverLocationUpdate,
    RideOffer,
    HeartBeat,
    SystemMessage,
}

impl ToString for WSMsgType {
    fn to_string(&self) -> String {
        match self {
            WSMsgType::DriverLocationUpdate => "driver_location_update".to_string(),
            WSMsgType::RideOffer => "ride_offer".to_string(),
            WSMsgType::HeartBeat => "heart_beat".to_string(),
            WSMsgType::SystemMessage => "system_message".to_string(),
        }
    }
}

/// A flat envelope that carries common metadata and flattens payload fields
/// into the same object. Example JSON:
/// {"type":"driver_location_update","v":2,"ts":1733917200000,"lat":1.0,"lng":2.0}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope<T> {
    /// "type" is a reserved keyword in Rust, so we use serde rename to map it.
    #[serde(rename = "type")]
    pub message_type: String,
    /// Protocol version
    // pub v: u8,
    /// Timestamp in ms since epoch
    // pub ts: i64,
    /// Flatten payload fields into the top-level JSON object.
    #[serde(flatten)]
    pub data: T,
}

/// Constructors for envelopes, kept small and explicit.
impl<T> Envelope<T> {
    pub fn new(ty: WSMsgType, v: u8, ts: i64, data: T) -> Self {
        Self {
            message_type: serde_json::to_string(&ty)
                .unwrap()
                .trim_matches('"')
                .to_string(),
            data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverLocationV1 {
    pub latitude: f64,
    pub longitude: f64,
    pub driver_id: Uuid,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coord {
    pub lat: f64,
    pub lng: f64,
}

/// Server → Client: ride offer v2 with richer fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RideOffer {
    pub ride_id: Uuid,
    pub expires_in_sec: u16,
    pub pickup: Coord,
    pub dropoff: Coord,
    pub surge: Option<f32>,
}

/// Server → Client: heartbeat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerPong {
    pub nonce: Option<String>,
}