// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use anyhow::Error;
use bluetooth::adapter::AdapterEvent;
use commands::SharedState;
use tauri::Manager;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

mod bluetooth;
mod commands;
mod constants;

#[tokio::main]
async fn main() {
    let client_response = bluetooth::Client::new().await;
    let tauri_default = tauri::Builder::default();
    if let Ok(client) = client_response {
        println!("client is ok");
        let state_arc = SharedState(Arc::new(Mutex::new(client)));
        tauri_default
            .manage(state_arc)
            .setup(|app| {
                let handle = app.handle();
                let state_clone = Arc::clone(&app.state::<SharedState>().0);

                tauri::async_runtime::spawn(async move {
                    let mut event_stream;
                    {
                        let mut client = state_clone.lock().await;
                        event_stream =
                            client.adapter_mut().adaptor_event_stream().await.unwrap();
                    }

                    let label = "main";
                    let property_event = "adapter_info_update";
                    let device_event = "devices_update";
                    while let Some(event) = event_stream.next().await {
                        match event {
                            AdapterEvent::AdapterPropertyChanged(adapter_info) => {
                                println!("adapter updated");
                                let _ = handle.emit_to(label, property_event, adapter_info);
                            }
                            AdapterEvent::DevicesUpdated(devices, _) => {
                                let _ = handle.emit_to(label, device_event, devices);
                            }
                        }
                    }
                    Ok::<(), Error>(())
                });
                println!("setup done");
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                commands::adapter_info,
                commands::set_alias,
                commands::set_pairable,
                commands::set_powered,
                commands::set_discoverable,
                commands::set_discoverable_timeout,
                commands::discover_devices,
                commands::cancel_discovering,
                commands::known_devices,
                commands::connect,
                commands::disconnect,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    } else {
        tauri_default
            .setup(|app| {
                let handle = app.handle();
                let _ = handle.emit_to("main", "adapter_event", "not_found");
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }

    //    adapter.set_powered(true).await?;
}
