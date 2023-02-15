#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

mod app_usage;

use active_win_pos_rs::get_active_window;
use app_usage::{AppUsageManager, Process};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
//https://tauri.app/v1/guides/features/command

#[tauri::command]
fn get_processes() -> HashMap<String, Process> {
    let mutex = AppUsageManager::get_mutex();
    let mut app_mgr = mutex.lock().unwrap();

    app_mgr.procs_since_last_access.clear();
    app_mgr.apps.clone()
}

#[tauri::command]
fn get_process_list_update() -> HashMap<String, Process> {
    let mutex = AppUsageManager::get_mutex();
    let mut app_mgr = mutex.lock().unwrap();

    let res = app_mgr.procs_since_last_access.clone();
    app_mgr.procs_since_last_access.clear();
    res
}

#[tauri::command]
fn get_active_process() -> Option<Process> {
    if let Ok(win) = get_active_window() {
        let proc = Process {
            name: win.process_name,
            pid: win.process_id,
            window_title: win.title,
            usage: 0,
        };
        return Some(proc);
    }

    return None;
}
fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    tauri::Builder::default()
        .setup(|_app| {
            app_usage::start_monitoring();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_processes,
            get_active_process,
            get_process_list_update
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
