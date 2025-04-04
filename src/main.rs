use std::time::Instant;

use num_enum::TryFromPrimitive;
use overcrowd_core::{
    AdvFlag, AdvType, BLEAddress, BLEAddressType, BLEAdvertisedData, BLEAdvertisedDevice, BLEData,
    BLEServiceData, BleUuid, ManufactureData,
};

fn main() {
    let data = BLEData {
        device_id: "my device id".to_owned(),
        time_stamp: Instant::now(),
        ble_advertised_data: BLEAdvertisedData {
            adv_flags: Some(AdvFlag::DiscLimited),
            payload: "Some Payload".as_bytes().to_vec(),
            service_uuids: vec![BleUuid::Uuid16(5)],
            name: Some("My Device Name".to_owned()),
            tx_power: Some(5),
            service_data: Some(BLEServiceData {
                uuid: BleUuid::Uuid16(5),
                service_data: "Some Service Data".as_bytes().to_vec(),
            }),
            manufacture_data: Some(ManufactureData {
                company_identifier: 15,
                payload: vec![1, 2, 3, 4, 5],
            }),
        },
        ble_advertised_device: BLEAdvertisedDevice {
            addr: BLEAddress {
                addr: [1, 2, 3, 4, 5, 6],
                addr_type: BLEAddressType::Public,
            },
            adv_type: AdvType::ScanResponse,
            rssi: 5,
        },
    };
    println!("{:#?}", data)
}
