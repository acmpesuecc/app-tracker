#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap};

mod app_usage;

use active_win_pos_rs::get_active_window;
use app_usage::{AppUsageManager, Process};
//https://tauri.app/v1/guides/features/command

#[tauri::command]
fn get_processes() -> HashMap<String, Process> {
    let mutex = AppUsageManager::get_mutex();
    let app_mgr = mutex.lock().unwrap();

    app_mgr.apps.clone()
}

#[tauri::command]
fn get_process_list_update() -> HashMap<String, Process> {
    let mutex = AppUsageManager::get_mutex();
    let mut app_mgr = mutex.lock().unwrap();

    app_mgr.get_procs_since_last_access() 
}

#[tauri::command]
fn get_active_process() -> Option<Process> {
    if let Ok(win) = get_active_window() {
        let proc = Process {
            name : win.process_name,
            pid: win.process_id,
            window_title: win.title,
            usage: 0
        };
        return Some(proc);
    }

    return None
}
fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            app_usage::start_monitoring();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_processes, get_active_process, get_process_list_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
