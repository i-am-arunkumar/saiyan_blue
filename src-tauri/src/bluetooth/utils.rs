use std::collections::{HashMap, HashSet};

use bluer::{Uuid, UuidExt};

use crate::constants;

pub fn get_service_class_name(bt_class: u32) -> Option<Vec<String>> {
    if bt_class == *constants::ERROR {
        return None;
    }
    let masked = bt_class & 0xFFE000;
    let mut classes = vec![];
    for (&bitmask, &name) in constants::CLASS_SERVICES.iter() {
        if masked & bitmask != 0 {
            let _ = &classes.push(name.to_string());
        }
    }
    if !classes.is_empty() {
        return Some(classes);
    }
    None
}

pub fn get_device_class_name_major(bt_class: u32) -> Option<String> {
    if bt_class == *constants::ERROR {
        return None;
    }

    let masked = bt_class & 0x1F00;
    if let Some(device_class_name) = constants::CLASS_DEVICES_MAJOR.get(&masked) {
        return Some(device_class_name.to_string());
    }

    None
}

pub fn get_device_class_name_minor(bt_class: u32) -> Option<String> {
    if bt_class == *constants::ERROR {
        return None;
    }

    let masked = bt_class & 0x1FFC;
    if let Some(device_class_name) = constants::CLASS_DEVICES_MINOR.get(&masked) {
        return Some(device_class_name.to_string());
    }

    None
}

fn is_reserved(uuid: &Uuid) -> bool {
    let mask_uuid = Uuid::parse_str("FFFF0000-0000-FFFF-FFFF-FFFFFFFFFFFF").unwrap();
    let reserved_uuid = Uuid::parse_str("00000000-0000-1000-8000-00805F9B34FB").unwrap();

    (uuid.as_u128() & mask_uuid.as_u128()) == reserved_uuid.as_u128()
}

pub fn get_uuid_local_service_names(uuids: HashSet<Uuid>) -> HashMap<String, String> {
    let uuid_map: HashMap<String, String> = uuids
        .iter()
        .map(|uuid| {
            let service_name = if let Some(short_uuid) = uuid.as_u16() {
                if !is_reserved(uuid) {
                    if short_uuid == 0 {
                        "Audio and input profiles"
                    } else {
                        "Proprietary"
                    }
                } else {
                    constants::UUID_SERVICES
                        .get(&short_uuid)
                        .unwrap_or(&"Unknown")
                }
            } else {
                "Unknown"
            };
            (uuid.to_string(), service_name.to_string())
        })
        .collect();

    uuid_map
}
