use std::{cmp::Ordering, collections::HashMap};

use anyhow::Result;
use bluer::{Address, DeviceProperty};

use super::device::{Device, DeviceInfo};

pub struct DeviceList {
    base: HashMap<Address, Device>,
    adapter_handle: bluer::Adapter,
}

impl DeviceList {
    pub async fn new(adapter: bluer::Adapter) -> Result<Self> {
        let device_addresses = adapter.device_addresses().await?;
        let base = futures::future::join_all(device_addresses.into_iter().map(|address| {
            let device = adapter.device(address).unwrap();
            Device::new(device)
        }))
        .await
        .into_iter()
        .map(|device| {
            let device = device.unwrap();
            (device.address().clone(), device)
        })
        .collect();
        Ok(Self {
            base,
            adapter_handle: adapter,
        })
    }

    pub async fn add_device(&mut self, address: Address) -> Result<DeviceInfo> {
        //self.adapter.adapter_handle.device_addresses()
        let device = self.base.get_mut(&address);
        let device_info = if let Some(device) = device {
            // device property changed
            device.refresh_properties().await?.clone()
        } else {
            // new device found
            let device_handle = self.adapter_handle.device(address).unwrap();
            let device = Device::new(device_handle).await?;
            let device_info = device.device_info().clone();
            self.base.insert(address, device);
            device_info
        };
        Ok(device_info)
    }

    pub async fn remove_device(&mut self, address: Address) -> Option<DeviceInfo> {
        let device = self.base.remove(&address);
        device.and_then(|device| Some(device.device_info().clone()))
    }

    pub fn update_device(
        &mut self,
        address: Address,
        property: DeviceProperty,
    ) -> Option<DeviceInfo> {
        self.base
            .get_mut(&address)
            .map(|device| device.update_device_property(property))
    }

    pub fn get(&self, address: &Address) -> Option<&Device> {
        self.base.get(address)
    }

    pub fn list(&self) -> Vec<DeviceInfo> {
        let mut devices = self
            .base
            .iter()
            .map(|device| device.1.device_info().clone())
            .collect::<Vec<DeviceInfo>>();
        devices.sort_by(|a, b| {
            if a.is_connected == b.is_connected {
                a.name.cmp(&b.name)
            } else {
                if a.is_connected {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        });
        devices
    }
}
