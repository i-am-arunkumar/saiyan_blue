use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::{bail, Ok, Result};
use bluer::{
    self, Adapter as BlueZAdapter, AdapterEvent as BlueZAdapterEvent, AdapterProperty, Address,
};
use futures::{
    lock::{Mutex, MutexGuard},
    stream::SelectAll,
    Stream,
};
use serde::Serialize;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use super::{
    device::DeviceInfo,
    device_list::DeviceList,
    utils::{
        get_device_class_name_major, get_device_class_name_minor, get_service_class_name,
        get_uuid_local_service_names,
    },
};

#[derive(Error, Debug)]
pub enum AdapterErrors {
    #[error("No adapters found in the system")]
    AdapterNotFound,
    #[error("Target adapter {0} is not powered on")]
    AdapterNotPoweredOn(String),
    #[error("Target adapter {0} is discovering")]
    AdapterIsDiscovering(String),
    #[error(
        "Adapter has a consumer already consuming the event, cannot establish another consumer"
    )]
    EventConsumerExist,
    #[error("Device is not found")]
    DeviceNotFound,
}

#[derive(Debug)]
pub enum DeviceEvent {
    DeviceAdded(DeviceInfo),
    DeviceRemoved(DeviceInfo),
    DeviceUpdated(DeviceInfo),
}
#[derive(Debug)]
pub enum AdapterEvent {
    AdapterPropertyChanged(AdapterInfo),
    DevicesUpdated(Vec<DeviceInfo>, DeviceEvent),
}

// Representation bluetooth adapter
#[derive(Debug, Clone)]
pub struct Adapter {
    name: String,
    adapter_handle: BlueZAdapter,
    adapter_info: Arc<Mutex<AdapterInfo>>,
    discovering: Arc<Mutex<bool>>,
    known_devices: Arc<Mutex<DeviceList>>,
    tx: Option<tokio::sync::mpsc::Sender<AdapterEvent>>,
}

impl Adapter {
    pub async fn new(adapter: BlueZAdapter) -> Result<Adapter> {
        let properties = adapter.all_properties().await?;
        let name = adapter.name().to_string();

        let adapter_info = AdapterInfo {
            name: name.clone(),
            ..AdapterInfo::from_properties(properties)
        };

        Ok(Adapter {
            name,
            known_devices: Arc::new(Mutex::new(DeviceList::new(adapter.clone()).await?)),
            adapter_handle: adapter,
            discovering: Arc::new(Mutex::new(adapter_info.discovering)),
            adapter_info: Arc::new(Mutex::new(adapter_info)),
            tx: None,
        })
    }

    pub async fn adaptor_event_stream(&mut self) -> Result<impl Stream<Item = AdapterEvent>> {
        if self.tx.is_some() {
            bail!(AdapterErrors::EventConsumerExist)
        }

        println!("adding event stream");


        let (tx, rx) = mpsc::channel::<AdapterEvent>(1);
        let event_tx = tx.clone();
        self.tx = Some(tx);
        let mut adapter_event_stream = self.adapter_handle.events().await?;
        let device_list_arc = Arc::clone(&self.known_devices);
        let adapter_info_arc = Arc::clone(&self.adapter_info);
        tokio::spawn(async move {
            while let Some(event) = adapter_event_stream.next().await {
                if event_tx.is_closed() {
                    break;
                }
                match event {
                    BlueZAdapterEvent::DeviceAdded(address) => {
                        let mut device_list = device_list_arc.lock().await;
                        let device_info = device_list.add_device(address).await?;
                        let _ = event_tx.send(AdapterEvent::DevicesUpdated(
                            device_list.list(),
                            DeviceEvent::DeviceAdded(device_info),
                        )).await;
                    }
                    BlueZAdapterEvent::DeviceRemoved(address) => {
                        let mut device_list = device_list_arc.lock().await;
                        let device_info =
                            device_list.remove_device(address).await.unwrap_or_default();
                        event_tx.send(AdapterEvent::DevicesUpdated(
                            device_list.list(),
                            DeviceEvent::DeviceRemoved(device_info),
                        )).await;
                    }
                    BlueZAdapterEvent::PropertyChanged(property) => {
                        let mut adapter_info = adapter_info_arc.lock().await;                        
                        adapter_info.update_property(property);
                        let res = event_tx
                            .send(AdapterEvent::AdapterPropertyChanged(adapter_info.clone())).await;
                        println!("property updated {:?}", res);

                    }
                }
            }
            println!("connection ccclosed");
            Ok::<()>(())
        });

        Ok(ReceiverStream::new(rx))
    }


    pub async fn adapter_info(&self) -> AdapterInfo {
        Arc::clone(&self.adapter_info).lock().await.clone()
    }

    pub async fn set_powered(&self, powered: bool) {
        let _ = self.adapter_handle.set_powered(powered).await;
    }

    pub async fn set_alias(&self, alias: String) {
        let _ = self.adapter_handle.set_alias(alias).await;
    }

    pub async fn set_pairable(&self, pairable: bool) {
        let _ = self.adapter_handle.set_pairable(pairable).await;
    }

    pub async fn set_discoverable(&self, discoverable: bool) {
        let _ = self.adapter_handle.set_discoverable(discoverable).await;
    }

    pub async fn set_discoverable_timeout(&self, discoverable_timeout: u32) {
        let _ = self
            .adapter_handle
            .set_discoverable_timeout(discoverable_timeout)
            .await;
    }

    pub async fn known_devices(&self) -> Vec<DeviceInfo> {
        self.known_devices.lock().await.list()
    }

    pub async fn cancel_discovering(&mut self) {
        let mut discovering = self.discovering.lock().await;
        println!("cancelling {:?}", discovering);
        if *discovering {
            *discovering = false;
        }
        println!("after cancelling {:?}", discovering);
    }

    pub async fn discover_devices(&mut self, timeout_in_seconds: u64) -> Result<()> {
        let mut discovering_guard = self.discovering.lock().await;

        if *discovering_guard {
            bail!(AdapterErrors::AdapterIsDiscovering(self.name.clone()));
        }

        if !self.adapter_info.lock().await.is_powered {
            bail!(AdapterErrors::AdapterNotPoweredOn(self.name.clone()));
        }

        *discovering_guard = true;

        drop(discovering_guard);
        

        let mut discovery = self.adapter_handle.discover_devices().await?;

        if let Some(tx) = self.tx.as_ref() {
            let mut changes = SelectAll::new();
            let timer = tokio::time::sleep(Duration::from_secs(timeout_in_seconds));
            let discovering_arc = Arc::clone(&self.discovering);
            let device_list_arc = Arc::clone(&self.known_devices);
            let tx = tx.clone();
            let adapter = self.adapter_handle.clone();
            tokio::spawn(async move {
                tokio::pin!(timer);

                loop {
                    if !*discovering_arc.lock().await {
                        break;
                    }
                    tokio::select! {
                        evt = discovery.next() => {
                            match evt {
                                Some(BlueZAdapterEvent::DeviceAdded(addr)) => {
                                    if let Result::Ok(dev) = adapter.device(addr) {
                                        if let Result::Ok(dev_evts) = dev.events().await {
                                            changes.push(dev_evts.map(move |bluer::DeviceEvent::PropertyChanged(property)|{(addr,property)}));
                                        }
                                    }
                                    let mut device_list = device_list_arc.lock().await;
                                    let device_info = device_list.add_device(addr).await?;
                                    let _ = tx.send(AdapterEvent::DevicesUpdated(device_list.list(), DeviceEvent::DeviceAdded(device_info))).await;
                                },
                                Some(BlueZAdapterEvent::DeviceRemoved(addr)) => {
                                    let mut device_list = device_list_arc.lock().await;
                                    let device_info = device_list.remove_device(addr).await.unwrap_or_default();
                                    let _ = tx.send(AdapterEvent::DevicesUpdated(device_list.list(),DeviceEvent::DeviceRemoved(device_info))).await;

                                },
                                Some(_) => (),
                                None => break,
                            }
                        },
                        Some((address, property)) = changes.next(), if !changes.is_empty() => {
                            let mut device_list = device_list_arc.lock().await;
                            let device_info = device_list.update_device(address, property).unwrap_or_default();
                            let _ = tx.send(AdapterEvent::DevicesUpdated(device_list.list(),DeviceEvent::DeviceUpdated(device_info))).await;
                        },
                        _ = &mut timer, if *discovering_arc.lock().await => {
                            let mut discovering_guard = discovering_arc.lock().await;
                            *discovering_guard = false;

                            println!("timeout");
                        },
                        () = tx.closed() => break,
                    }
                }
                println!("Connection closed");
                Ok::<()>(())
            });
        }

        Ok(())
    }

    pub async fn connect_device(&self, address: Address) -> Result<()> {
        if let Some(device) = self.known_devices.lock().await.get(&address) {
            device.connect().await
        } else {
            bail!(AdapterErrors::DeviceNotFound)
        }
    }

    pub async fn disconnect_device(&self, address: Address) -> Result<()> {
        if let Some(device) = self.known_devices.lock().await.get(&address) {
            device.disconnect().await
        } else {
            bail!(AdapterErrors::DeviceNotFound)
        }
    }
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct AdapterInfo {
    not_found: bool, //todo use Options for adapter instead of not found
    discovering: bool,
    address: String,
    address_type: String,
    alias: String,
    pairable: bool,
    name: String,
    is_discoverable: bool,
    is_powered: bool,
    system_name: String,
    pairable_timeout: u32,
    discoverable_timeout: u32,
    class: u32,
    device_major_name: String,
    device_minor_name: String,
    service_categories: Vec<String>,
    icon: String,
    active_advertising_instances: u8,
    supported_advertising_instances: u8,
    supported_advertising_system_includes: Vec<String>,
    supported_advertising_features: Vec<String>,
    max_advertisement_length: u8,
    max_scan_response_length: u8,
    min_tx_power: i16,
    max_tx_power: i16,
    uuids: HashMap<String, String>,
}

impl AdapterInfo {
    pub fn from_properties(properties: Vec<AdapterProperty>) -> Self {
        let mut adapter_info = AdapterInfo {
            not_found: false,
            ..AdapterInfo::default()
        };

        for property in properties {
            adapter_info.update_property(property);
        }

        adapter_info
    }

    pub fn update_property(&mut self, property: AdapterProperty) {
        match property {
            AdapterProperty::Address(address) => self.address = address.to_string(),
            AdapterProperty::AddressType(address_type) => {
                self.address_type = address_type.to_string()
            }
            AdapterProperty::SystemName(system_name) => self.system_name = system_name,
            AdapterProperty::Alias(alias) => self.alias = alias,
            AdapterProperty::Class(class) => {
                self.class = class;
                self.device_major_name =
                    get_device_class_name_major(class).unwrap_or(String::from("UNKNOWN"));
                self.device_minor_name =
                    get_device_class_name_minor(class).unwrap_or(String::from("UNKNOWN"));
                self.service_categories = get_service_class_name(class).unwrap_or(vec![])
            }
            AdapterProperty::Powered(is_powered) => self.is_powered = is_powered,
            AdapterProperty::Discoverable(is_discoverable) => {
                self.is_discoverable = is_discoverable
            }
            AdapterProperty::Pairable(pairable) => self.pairable = pairable,
            AdapterProperty::PairableTimeout(pairable_timeout) => {
                self.pairable_timeout = pairable_timeout
            }
            AdapterProperty::DiscoverableTimeout(discoverable_timeout) => {
                self.discoverable_timeout = discoverable_timeout
            }
            AdapterProperty::Discovering(scanning) => self.discovering = scanning,
            AdapterProperty::Uuids(uuids) => self.uuids = get_uuid_local_service_names(uuids),
            AdapterProperty::ActiveAdvertisingInstances(active_advertising_instances) => {
                self.active_advertising_instances = active_advertising_instances
            }
            AdapterProperty::SupportedAdvertisingInstances(supported_advertising_instances) => {
                self.supported_advertising_instances = supported_advertising_instances
            }
            AdapterProperty::SupportedAdvertisingSystemIncludes(
                supported_advertising_system_includes,
            ) => {
                let mut features = vec![];
                for feature in supported_advertising_system_includes {
                    features.push(feature.to_string())
                }
                self.supported_advertising_system_includes = features
            }
            AdapterProperty::SupportedAdvertisingSecondaryChannels(_) => (),
            AdapterProperty::SupportedAdvertisingCapabilities(capabilities) => {
                self.max_advertisement_length = capabilities.max_advertisement_length;
                self.max_scan_response_length = capabilities.max_scan_response_length;
                self.max_tx_power = capabilities.max_tx_power;
                self.min_tx_power = capabilities.min_tx_power
            }
            AdapterProperty::SupportedAdvertisingFeatures(supported_advertising_features) => {
                let mut features = vec![];
                for feature in supported_advertising_features {
                    features.push(feature.to_string())
                }
                self.supported_advertising_features = features
            }
            _ => (),
        }
    }
}
