use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

/// Configuration for a `Droid` instance.
#[derive(Debug, Clone)]
pub struct DroidConfig {
    /// The address and port of the ADB server.
    pub adb_server_addr: SocketAddrV4,
    /// The serial number of the specific device to connect to.
    /// If `None`, the first available device will be used.
    pub device_serial: Option<String>,
    /// The default timeout duration for operations like `wait_for`.
    pub default_timeout: Duration,
    /// The default polling interval for `wait_for` operations.
    pub default_interval: Duration,
    /// The default confidence threshold for image template matching (0.0 to 1.0).
    pub default_confidence: f32,
}

impl Default for DroidConfig {
    /// Provides a reasonable default configuration.
    /// - ADB Server: `127.0.0.1:5037`
    /// - Device: Auto-select first available
    /// - Timeout: 20 seconds
    /// - Interval: 0.5 seconds
    /// - Confidence: 0.8
    fn default() -> Self {
        Self {
            adb_server_addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5037),
            device_serial: None,
            default_timeout: Duration::from_secs(20),
            default_interval: Duration::from_millis(500),
            default_confidence: 0.8,
        }
    }
}

impl DroidConfig {
    /// Sets the ADB server address (IP and port).
    pub fn address(mut self, addr: SocketAddrV4) -> Self {
        self.adb_server_addr = addr;
        self
    }

    /// Sets the serial number of the specific device to connect to.
    pub fn serial(mut self, serial: String) -> Self {
        self.device_serial = Some(serial);
        self
    }

    /// Sets the default timeout duration.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Sets the default confidence threshold for image matching.
    /// The value will be clamped between 0.0 and 1.0.
    pub fn confidence(mut self, confidence: f32) -> Self {
        self.default_confidence = confidence.clamp(0.0, 1.0);
        self
    }
}
