use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::{Mutex, Once};
use std::{thread, time};

use active_win_pos_rs::get_active_window;
use serde::Serialize;

const UPDATE_INTERVAL: u64 = 1;

#[derive(Clone, Debug, Serialize)]
pub struct Process {
    pub name: String,
    pub window_title: String,
    pub pid: u64,
    pub usage: u16,
}

#[derive(Default)]
pub struct AppUsageManager {
    pub apps: HashMap<String, Process>,
    pub current_active_process: Option<Process>,
    pub procs_since_last_access: HashMap<String, Process>
}

impl AppUsageManager {
    pub fn get_mutex() -> Arc<Mutex<AppUsageManager>> {
        static mut SINGLETON: MaybeUninit<Arc<Mutex<AppUsageManager>>> =
            MaybeUninit::<Arc<Mutex<AppUsageManager>>>::uninit();
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let a = Arc::new(Mutex::new(AppUsageManager {
                    ..Default::default()
                }));

                SINGLETON.write(a);
            });

            SINGLETON.assume_init_ref().clone()
        }
    }
}

pub fn start_monitoring() {
    let mutex = AppUsageManager::get_mutex();

    thread::spawn(move || {
        loop {
            let mut app_mgr = mutex.lock().unwrap();

            if let Ok(window) = get_active_window() {
                if let Some(app) = app_mgr.apps.get_mut(&window.process_name) {
                    // is the current process there in our tracked processes?
                    app.usage += UPDATE_INTERVAL as u16;
                    app.window_title = window.title;

                    let new_proc = app.clone();

                    // update procs_since_last_access 
                    if let Some(proc) = app_mgr.procs_since_last_access.clone().get_mut(&new_proc.name.clone()) {
                        proc.usage = new_proc.usage;
                        proc.window_title = new_proc.window_title.clone();
                    } else {
                        app_mgr.procs_since_last_access.insert(new_proc.name.clone(), new_proc.clone());
                    }

                    let curr_proc = app_mgr.current_active_process.clone().unwrap();
                    
                    // if active window has been switched, update current_active_process
                    if curr_proc.name != window.process_name {
                        app_mgr.current_active_process = Some(new_proc);
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
                        app_mgr
                            .apps
                            .insert(window.process_name.clone(), new_proc.clone());
                        app_mgr.procs_since_last_access.insert(new_proc.name.clone(), new_proc.clone());
                        app_mgr.current_active_process = Some(new_proc);
                    }
                }
            }
            drop(app_mgr);
            thread::sleep(time::Duration::from_secs(UPDATE_INTERVAL));
        }
    });
}
