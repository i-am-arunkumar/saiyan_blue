use std::sync::Arc;

use bluer::Address;
use tauri::State;
use tokio::sync::Mutex;

use crate::bluetooth::{adapter::AdapterInfo, device::DeviceInfo, Client};

pub struct SharedState(pub Arc<Mutex<Client>>);

#[tauri::command]
pub async fn adapter_info(state: State<'_, SharedState>) -> Result<AdapterInfo, ()> {
    let state_clone = Arc::clone(&state.0);
    let client: tokio::sync::MutexGuard<Client> = state_clone.lock().await;
    let adapter_info = client.adapter().adapter_info().await;
    Ok(adapter_info)
}

#[tauri::command]
pub async fn set_alias(alias: String, state: State<'_, SharedState>) -> Result<bool, ()> {
    let state_clone = Arc::clone(&state.0);
    let client = state_clone.lock().await;
    client.adapter().set_alias(alias).await;
    Ok(true)
}

#[tauri::command]
pub async fn set_powered(powered: bool, state: State<'_, SharedState>) -> Result<bool, ()> {
    let state_clone = Arc::clone(&state.0);
    let client = state_clone.lock().await;
    client.adapter().set_powered(powered).await;
    Ok(true)
}

#[tauri::command]
pub async fn set_discoverable(
    discoverable: bool,
    state: State<'_, SharedState>,
) -> Result<bool, ()> {
    let state_clone = Arc::clone(&state.0);
    let client = state_clone.lock().await;
    client.adapter().set_discoverable(discoverable).await;
    Ok(true)
}

#[tauri::command]
pub async fn set_discoverable_timeout(
    discoverable_timeout: u32,
    state: State<'_, SharedState>,
) -> Result<bool, ()> {
    let state_clone = Arc::clone(&state.0);
    let client = state_clone.lock().await;
    client
        .adapter()
        .set_discoverable_timeout(discoverable_timeout)
        .await;
    Ok(true)
}

#[tauri::command]
pub async fn set_pairable(pairable: bool, state: State<'_, SharedState>) -> Result<bool, ()> {
    let state_clone = Arc::clone(&state.0);
    let client = state_clone.lock().await;
    client.adapter().set_pairable(pairable).await;
    Ok(true)
}

#[tauri::command]
pub async fn known_devices(state: State<'_, SharedState>) -> Result<Vec<DeviceInfo>, String> {
    let state_clone = Arc::clone(&state.0);
    let client = state_clone.lock().await;
    let result = client.adapter().known_devices().await;
    Ok(result)
}

#[tauri::command]
pub async fn cancel_discovering(state: State<'_, SharedState>) -> Result<(), ()> {
    let state_clone = Arc::clone(&state.0);
    let mut client = state_clone.lock().await;
    client.adapter_mut().cancel_discovering().await;
    Ok(())
}

#[tauri::command]
pub async fn discover_devices(timeout: u64, state: State<'_, SharedState>) -> Result<(), String> {
    let state_clone = Arc::clone(&state.0);
    let mut client = state_clone.lock().await;

    if let Err(err) = client.adapter_mut().discover_devices(timeout).await {
        Err(err.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub async fn connect(address: [u8; 6], state: State<'_, SharedState>) -> Result<(), String> {
    let state_clone = Arc::clone(&state.0);
    let mut client = state_clone.lock().await;

    if let Err(err) = client
        .adapter_mut()
        .connect_device(Address::new(address))
        .await
    {
        Err(err.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub async fn disconnect(address: [u8; 6], state: State<'_, SharedState>) -> Result<(), String> {
    let state_clone = Arc::clone(&state.0);
    let mut client = state_clone.lock().await;

    if let Err(err) = client
        .adapter_mut()
        .disconnect_device(Address::new(address))
        .await
    {
        Err(err.to_string())
    } else {
        Ok(())
    }
}
