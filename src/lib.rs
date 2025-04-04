use std::time::Instant;

use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub struct BLEData {
    device_id: String,
    time_stamp: Instant,
    ble_advertised_data: BLEAdvertisedData,
    ble_advertised_device: BLEAdvertisedDevice,
}

pub struct BLEAdvertisedData {
    adv_flags: Option<AdvFlag>,
    payload: Vec<u8>,
    service_uuids: Vec<BleUuid>,
    name: Option<String>,
    tx_power: Option<u8>,
    service_data: Option<BLEServiceData>,
    manufacture_data: Option<ManufactureData>,
}

#[derive(Debug, Clone)]
pub struct ManufactureData {
    pub company_identifier: u16,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BLEServiceData {
    pub uuid: BleUuid,
    pub service_data: Vec<u8>,
}

/// A Bluetooth UUID.
#[derive(Copy, Clone, Debug)]
pub enum BleUuid {
    /// A 16-bit UUID.
    Uuid16(u16),
    /// A 32-bit UUID.
    Uuid32(u32),
    /// A 128-bit UUID.
    Uuid128([u8; 16]),
}

bitflags! {
  #[repr(transparent)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct AdvFlag: u8 {
    /// LE Limited Discoverable Mode
    const DiscLimited = 1 as _;
    /// LE General Discoverable Mode
    const DiscGeneral = 2 as _;
    /// BR/EDR Not Supported
    const BrEdrUnsupported = 4 as _;
    /// Simultaneous LE and BR/EDR to Same Device Capable (Controller)
    const SimultaneousController = 0b01000;
    /// Simultaneous LE and BR/EDR to Same Device Capable (Host)
    const SimultaneousHost       = 0b10000;
  }
}

pub struct BLEAdvertisedDevice {
    addr: BLEAddress,
    adv_type: AdvType,
    rssi: i8,
    // sid: u8
    // prim_phy: PrimPhy,
    // sec_phy: Option<SecPhy>,
    // periodic_itvl: u16
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
pub enum PrimPhy {
    /// 1Mbps phy
    Phy1M = 1 as _,
    /// Coded phy
    Coded = 3 as _,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AdvType {
    /// indirect advertising
    Ind,
    /// direct advertising
    DirectInd,
    /// indirect scan response
    ScanInd,
    /// indirect advertising - not connectable
    NonconnInd,
    ScanResponse,
    // #[cfg(esp_idf_bt_nimble_ext_adv)]
    // Extended(u8),
}

pub struct BLEAddress {
    addr: [u8; 6],
    addr_type: BLEAddressType,
}

/// Bluetooth Device address type
#[derive(PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum BLEAddressType {
    Public = 0 as _,
    Random = 1 as _,
    PublicID = 2 as _,
    RandomID = 3 as _,
}
