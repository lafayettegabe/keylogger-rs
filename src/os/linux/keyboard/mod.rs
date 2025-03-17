use crate::utils::log_manager::LogManager;
use chrono::{DateTime, Local};
use evdev::{Device, KeyCode};
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::SystemTime;

pub fn is_real_keyboard(device: &Device) -> bool {
    device.supported_events().contains(evdev::EventType::KEY)
        && device.supported_keys().map_or(false, |keys| {
            keys.contains(KeyCode::KEY_A) && keys.contains(KeyCode::KEY_ENTER)
        })
}

pub fn detect_keyboard() -> Result<Device, Box<dyn std::error::Error>> {
    println!("Searching for keyboard devices...");
    let devices = fs::read_dir("/dev/input/")?;

    for entry in devices {
        let path = entry?.path();
        if path.to_string_lossy().contains("event") {
            match Device::open(&path) {
                Ok(dev) => {
                    if is_real_keyboard(&dev) {
                        println!(
                            "Found keyboard device: {:?} ({})",
                            path,
                            dev.name().unwrap_or("Unknown")
                        );
                        return Ok(dev);
                    }
                }
                Err(e) => println!("Skipping device {:?}: {}", path, e),
            }
        }
    }

    Err("No keyboard device found".into())
}

fn get_timestamp() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Local> = now.into();
    datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

pub fn listen(
    mut device: Device,
    log_manager: Arc<Mutex<LogManager>>,
) -> Result<(), Box<dyn std::error::Error>> {
    device.set_nonblocking(true)?;

    {
        let mut lm = log_manager.lock().unwrap();
        lm.write_line(&format!(
            "[{}] Starting keyboard monitoring",
            get_timestamp()
        ))?;
    }

    println!("Keyboard monitoring started. Press Ctrl+C to stop.");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Received stop signal. Shutting down...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        match device.fetch_events() {
            Ok(events) => {
                for event in events {
                    let message = match event.destructure() {
                        evdev::EventSummary::Key(_, code, 1) => {
                            format!("[{}] Pressed: {:?}", get_timestamp(), code)
                        }
                        evdev::EventSummary::Key(_, code, 0) => {
                            format!("[{}] Released: {:?}", get_timestamp(), code)
                        }
                        _ => continue,
                    };

                    if let Ok(mut lm) = log_manager.lock() {
                        if let Err(e) = lm.write_line(&message) {
                            eprintln!("Error writing to log: {}", e);
                        }
                    }
                }
            }
            Err(_) => {
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }

    if let Ok(mut lm) = log_manager.lock() {
        lm.write_line(&format!(
            "[{}] Keyboard monitoring stopped",
            get_timestamp()
        ))?;

        if let Err(e) = lm.rotate() {
            eprintln!("Final rotation error: {}", e);
        }
    }

    println!("Keyboard monitoring stopped.");
    Ok(())
}
