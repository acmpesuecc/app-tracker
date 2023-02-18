#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app_tracker;

use std::sync::{Arc, Mutex};

use app_tracker::{
    get_current_app, get_tracked_apps, start_monitoring, AppTracker
};

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn main() {
    let app_tracker = AppTracker::new();
    let mutex = Arc::new(Mutex::new(app_tracker));

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    tauri::Builder::default()
        .manage(mutex.clone())
        .setup(move |app| {
            let app_handle = app.app_handle();
            start_monitoring(mutex.clone(), move|tracker_update| {
                if let Some(win) = app_handle.get_window("main") {
                    win.emit("tracking-update", tracker_update).unwrap();
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_tracked_apps,
            get_current_app,
        ])
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                if app.get_window("main").is_none() {
                    tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("/".into()))
                        .title("app-tracker")
                        .build()
                        .unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
