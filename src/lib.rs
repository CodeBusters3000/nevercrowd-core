use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLEData {
    pub device_id: String,
    pub time_stamp: DateTime<Utc>,
    pub company_identifier: Option<u16>,
    pub rssi: i8,
    pub model_number: String,
    pub addr: [u8; 6],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyBLEData {
    pub device_id: String,
    pub time_stamp: DateTime<Utc>,
    pub company_identifier: Option<u16>,
    pub rssi: i8,
    pub addr: [u8; 6],
}
