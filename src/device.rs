use std::net::SocketAddrV4;
use std::time::Duration;

use adb_client::{ADBDeviceExt, ADBServer, ADBServerDevice};
use image::DynamicImage;

use crate::common::point::Point;
use crate::error::{DroidError, Result};

pub struct DeviceController {
    device: ADBServerDevice,
}

impl DeviceController {
    /// 创建一个新的设备控制器。
    /// 如果 `device_identifier` 是 Some，则连接到指定设备。
    /// 如果是 None，则自动连接到第一个可用的设备。
    pub fn new(device_identifier: Option<&str>, adb_addr: SocketAddrV4) -> Result<Self> {
        let mut server = ADBServer::new(adb_addr);

        let devices = server
            .devices()
            .map_err(|e| DroidError::AdbError(e.to_string()))?;
        if devices.is_empty() {
            return Err(DroidError::DeviceNotFound);
        }

        let target_identifier = match device_identifier {
            Some(identifier) => devices
                .iter()
                .find(|d| d.identifier == identifier)
                .map(|d| d.identifier.clone())
                .ok_or(DroidError::DeviceNotFound)?,
            None => devices[0].identifier.clone(),
        };

        log::info!(
            "Connecting to device '{}' via ADB server at {}",
            target_identifier,
            adb_addr
        );
        let device = server
            .get_device_by_name(&target_identifier)
            .map_err(|e| DroidError::AdbError(e.to_string()))?;

        Ok(Self { device })
    }

    /// Executes a raw shell command string.
    /// This is the new, safer internal method.
    fn shell(&mut self, command: &str) -> Result<String> {
        log::debug!("Executing ADB shell command: {}", command);
        let args: Vec<&str> = command.split_whitespace().collect();
        let mut output_buffer: Vec<u8> = Vec::new();

        self.device
            .shell_command(&args, &mut output_buffer)
            .map_err(|e| DroidError::AdbError(e.to_string()))?;

        String::from_utf8(output_buffer)
            .map_err(|e| DroidError::AdbError(format!("Shell output is not valid UTF-8: {}", e)))
    }

    /// 截取当前设备屏幕
    pub fn screenshot(&mut self) -> Result<DynamicImage> {
        log::debug!("Capturing screenshot...");
        let png_data = self
            .device
            .framebuffer_bytes()
            .map_err(|e| DroidError::AdbError(e.to_string()))?;

        image::load_from_memory(&png_data).map_err(DroidError::ImageError)
    }

    /// Taps a point on the screen.
    pub fn tap(&mut self, point: Point) -> Result<()> {
        let cmd = format!("input tap {} {}", point.x, point.y);
        self.shell(&cmd)?;
        Ok(())
    }

    /// 在屏幕上滑动
    pub fn swipe(&mut self, start: Point, end: Point, duration: Duration) -> Result<()> {
        let cmd = format!(
            "input swipe {} {} {} {} {}",
            start.x,
            start.y,
            end.x,
            end.y,
            duration.as_millis()
        );
        self.shell(&cmd)?;
        Ok(())
    }

    /// Inputs text.
    pub fn input_text(&mut self, text: &str) -> Result<()> {
        // ADB shell requires escaping spaces. %s is a common way.
        let escaped_text = text.replace(' ', "%s");
        let cmd = format!("input text {}", escaped_text);
        self.shell(&cmd)?;
        Ok(())
    }

    /// 发送一个按键事件
    pub fn input_keyevent(&mut self, key_code: i32) -> Result<()> {
        let cmd = format!("input keyevent {}", key_code);
        self.shell(&cmd)?;
        Ok(())
    }
}
