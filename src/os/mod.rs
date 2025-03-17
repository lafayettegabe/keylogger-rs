#[cfg(all(feature = "linux", target_os = "linux"))]
pub mod linux;

pub mod windows {
    use crate::utils::errors::OsError;
    use crate::utils::log_manager::LogManager;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    pub fn handle_windows(
        _log_manager: Arc<Mutex<LogManager>>,
        _duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(OsError::UnsupportedFeature))
    }
}

pub mod mac {
    use crate::utils::errors::OsError;
    use crate::utils::log_manager::LogManager;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    pub fn handle_mac(
        _log_manager: Arc<Mutex<LogManager>>,
        _duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(OsError::UnsupportedFeature))
    }
}
