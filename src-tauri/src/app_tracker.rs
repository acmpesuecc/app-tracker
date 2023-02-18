use core::time;
use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}, thread, sync::{Arc, Mutex}};

use active_win_pos_rs::get_active_window;
use serde::Serialize;
use tauri::State;

const UPDATE_INTERVAL: u64 = 3;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Process {
    pub name: String,
    pub window_title: String,
    pub pid: u64,
    pub usage: u16,
}

#[derive(Default)]
pub struct AppTracker {
    pub tracked_apps: HashMap<String, Process>,
    pub updated_procs: HashMap<String, Process>,
    pub current_app: Option<Process>,
    pub session_start_timestamp: u64,
}

impl AppTracker {
    pub fn new() -> AppTracker {
        AppTracker {
            session_start_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            ..Default::default()
        }
    }
}
#[tauri::command]
pub fn get_tracked_apps(app_tracker : State<Arc<Mutex<AppTracker>>>) -> HashMap<String, Process> {
    let a = app_tracker.lock().unwrap();

    a.tracked_apps.clone()
}

#[tauri::command]
pub fn get_current_app() -> Process {
    let win = get_active_window().unwrap();

    Process {
        name: win.process_name,
        window_title: win.title,
        pid: win.process_id,
        ..Default::default()
    }
}

pub fn start_monitoring<F>(app_tracker_mutex: Arc<Mutex<AppTracker>>, callback: F) -> ()
where 
    F: Fn(HashMap<String, Process>) -> () + Sync + Send + 'static
{
    thread::spawn(move || {
        loop {
            let mut app_tracker = app_tracker_mutex.lock().unwrap();

            if let Ok(window) = get_active_window() {
                if let Some(app) = app_tracker.tracked_apps.get_mut(&window.process_name) {
                    // is the current process there in our tracked processes?
                    app.usage += UPDATE_INTERVAL as u16;
                    app.window_title = window.title;
                    app.pid = window.process_id;

                    let new_proc = app.clone();

                    // update procs_since_last_access
                    if let Some(mut proc) = app_tracker
                        .updated_procs
                        .get_mut(&new_proc.name.clone())
                    {
                        proc.usage = new_proc.usage;
                        proc.window_title = new_proc.window_title.clone();
                        proc.pid = new_proc.pid;
                    } else {
                        app_tracker
                            .updated_procs
                            .insert(new_proc.name.clone(), new_proc.clone());
                    }

                    let curr_proc = app_tracker.current_app.clone().unwrap();

                    // if active window has been switched, update current_active_process
                    if curr_proc.name != window.process_name {
                        app_tracker.current_app = Some(new_proc);
                    }
                } else {
                    // if this process isn't already being tracked

                    // ignore application frame host (UWP apps)
                    if window.process_name.to_lowercase() != "application frame host" {
                        // add the new process, and set it as the current active process
                        let new_proc = Process {
                            pid: window.process_id,
                            name: window.process_name.clone(),
                            window_title: window.title.clone(),
                            usage: 1,
                        };
                        app_tracker
                            .tracked_apps
                            .insert(window.process_name.clone(), new_proc.clone());
                        app_tracker
                            .updated_procs
                            .insert(new_proc.name.clone(), new_proc.clone());
                        app_tracker.current_app = Some(new_proc);
                    }
                }
            }
            
            callback(app_tracker.updated_procs.clone());
            app_tracker.updated_procs.clear();
            
            drop(app_tracker);
            thread::sleep(time::Duration::from_secs(UPDATE_INTERVAL));
        }
    });
}
