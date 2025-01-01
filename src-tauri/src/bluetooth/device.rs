use std::collections::HashMap;

use super::utils::{
    get_device_class_name_major, get_device_class_name_minor, get_service_class_name,
    get_uuid_local_service_names,
};
use anyhow::{Ok, Result};
use bluer::{self, Address, Device as BlueZDevice, DeviceProperty};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Device {
    address: Address,
    info: DeviceInfo,
    device_handle: BlueZDevice,
}

impl Device {
    pub async fn new(device: BlueZDevice) -> Result<Self> {
        let device_properties = device.all_properties().await?;
        let mut device_info = DeviceInfo::from_properties(device_properties);
        device_info.discovered_adapter_name = device.adapter_name().into();
        device_info.address_string = device.address().to_string();
        device_info.address = device.address().0;
        Ok(Self {
            address: device.address(),
            info: device_info,
            device_handle: device,
        })
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn device_info(&self) -> &DeviceInfo {
        &self.info
    }

    pub fn update_device_property(&mut self, property: DeviceProperty) -> DeviceInfo {
        self.info.update_property(property);
        self.info.clone()
    }

    pub async fn refresh_properties(&mut self) -> Result<&DeviceInfo> {
        for property in self.device_handle.all_properties().await? {
            self.info.update_property(property);
        }
        Ok(&self.info)
    }

    pub async fn connect(&self) -> Result<()> {
        self.device_handle.connect().await?;
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        self.device_handle.disconnect().await?;
        Ok(())
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct DeviceInfo {
    discovered_adapter_name: String,
    pub name: String,
    alias: String,
    address: [u8; 6],
    pub address_string: String,
    address_type: String,
    class: u32,
    device_major_name: String,
    device_minor_name: String,
    service_categories: Vec<String>,
    uuids: HashMap<String, String>,
    is_paired: bool,
    pub is_connected: bool,
    is_trusted: bool,
    is_blocked: bool,
    is_wake_allowed: bool,
    is_legacy_pairing: bool,
    battery_percentage: u8,
}

impl DeviceInfo {
    fn from_properties(device_properties: Vec<DeviceProperty>) -> Self {
        let mut device_info = DeviceInfo::default();
        for property in device_properties {
            device_info.update_property(property);
        }
        device_info
    }

    fn update_property(&mut self, property: DeviceProperty) {
        match property {
            DeviceProperty::Name(device_name) => self.name = device_name,
            DeviceProperty::AddressType(address_type) => {
                self.address_type = address_type.to_string()
            }
            DeviceProperty::Icon(_) => { /* ignored */ }
            DeviceProperty::Class(class) => {
                self.class = class;
                self.device_major_name =
                    get_device_class_name_major(class).unwrap_or(String::from("UNKNOWN"));
                self.device_minor_name =
                    get_device_class_name_minor(class).unwrap_or(String::from("UNKNOWN"));
                self.service_categories = get_service_class_name(class).unwrap_or(vec![])
            }
            DeviceProperty::Appearance(_) => { /* ignored */ }
            DeviceProperty::Uuids(uuids) => self.uuids = get_uuid_local_service_names(uuids),
            DeviceProperty::Paired(paired) => self.is_paired = paired,
            DeviceProperty::Connected(connected) => self.is_connected = connected,
            DeviceProperty::Trusted(trusted) => self.is_trusted = trusted,
            DeviceProperty::Blocked(blocked) => self.is_blocked = blocked,
            DeviceProperty::WakeAllowed(wake_allowed) => self.is_wake_allowed = wake_allowed,
            DeviceProperty::Alias(alias) => self.alias = alias,
            DeviceProperty::LegacyPairing(legacy_pairing) => {
                self.is_legacy_pairing = legacy_pairing
            }
            DeviceProperty::Modalias(_) => { /* ignored */ }
            DeviceProperty::Rssi(_) => { /* ignored */ }
            DeviceProperty::TxPower(_) => { /* ignored */ }
            DeviceProperty::ManufacturerData(_) => { /* ignored */ }
            DeviceProperty::ServiceData(_) => { /* ignored */ }
            DeviceProperty::ServicesResolved(_) => { /* ignored */ }
            DeviceProperty::AdvertisingFlags(_) => { /* ignored */ }
            DeviceProperty::AdvertisingData(_) => { /* ignored */ }
            DeviceProperty::BatteryPercentage(percent) => self.battery_percentage = percent,
            _ => todo!(),
        }
    }
}
