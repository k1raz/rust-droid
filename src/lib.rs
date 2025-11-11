//! A simple and fluent Android automation library for Rust.
//!
//! `rust-droid` provides a high-level API to control Android devices via ADB
//! (Android Debug Bridge). It is inspired by popular automation tools like
//! Airtest, with a focus on ease of use and a fluent, builder-style API.
//!
//! # Quick Start
//!
//! ```no_run
//! use rust_droid::{Droid, DroidConfig, Target, models::KeyCode};
//! use std::path::PathBuf;
//! use std::time::Duration;
//!
//! fn main() -> anyhow::Result<()> {
//!     // 1. Create a Droid instance.
//!     let mut droid = Droid::new(DroidConfig::default())?;
//!
//!     // 2. Define a target image.
//!     let settings_icon = Target::from("path/to/your/settings_icon.png");
//!
//!     // 3. Wait for the icon to appear and get its position.
//!     let icon_position = droid.wait_for(settings_icon).execute()?;
//!
//!     // 4. Tap the icon.
//!     droid.touch(icon_position.into()).execute()?;
//!
//!     // 5. Press the back button.
//!     droid.keyevent(KeyCode::Back).execute()?;
//!
//!     Ok(())
//! }
//! ```

pub mod action;
pub mod common;
pub mod config;
pub mod device;
pub mod error;
pub mod models;
pub mod vision;

use crate::common::point::Point;
use crate::common::rect::Rect;
use crate::common::relative_rect::RelativeRect;
use crate::models::KeyCode;
pub use config::DroidConfig;
use device::DeviceController;
use error::{DroidError, Result};
use image::{DynamicImage, GenericImageView};
pub use models::Target;
use std::path::Path;
use std::time::Duration;

/// The main entry point for interacting with an Android device.
///
/// The `Droid` struct holds the connection to a device and provides methods
/// for performing actions like tapping, swiping, and image recognition.
pub struct Droid {
    controller: DeviceController,
    pub(crate) config: DroidConfig,
}

impl Droid {
    /// Creates a new `Droid` instance and connects to a device.
    pub fn new(config: DroidConfig) -> Result<Self> {
        let controller =
            DeviceController::new(config.device_serial.as_deref(), config.adb_server_addr)?;
        Ok(Self { controller, config })
    }

    pub(crate) fn resolve_target(
        &mut self,
        target: &Target,
        threshold: f32,
        search_rect: Option<RelativeRect>,
    ) -> Result<Point> {
        match target {
            Target::Point(p) => {
                if search_rect.is_some() {
                    log::warn!("Search region is ignored when the target is a Point.");
                }
                log::debug!("Target resolved to a direct point: {:?}", p);
                Ok(*p)
            }
            Target::Image(path) => {
                log::debug!("Attempting to resolve image target: {:?}", path);
                let needle = image::open(path)?;
                let haystack = self.controller.screenshot()?;

                let absolute_search_rect: Option<Rect> = search_rect.map(|relative_rect| {
                    let (w, h) = haystack.dimensions();
                    relative_rect.to_absolute(w, h)
                });

                let match_result = vision::find_template(
                    &haystack,
                    &needle,
                    threshold,
                    path,
                    absolute_search_rect,
                )?;

                let center_point = match_result.rect.center();
                log::info!(
                    "Image target found at {:?}, center: {:?}, confidence: {:.4}",
                    match_result.rect,
                    center_point,
                    match_result.confidence
                );
                Ok(center_point)
            }
        }
    }

    /// Initiates a touch action on a target.
    ///
    /// Returns a `TouchBuilder` to configure and execute the action.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_droid::{Droid, DroidConfig, Target};
    /// # let mut droid = Droid::new(DroidConfig::default()).unwrap();
    /// # let target = Target::from("path/to/image.png");
    /// // Tap an image target twice.
    /// droid.touch(target).times(2).execute()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn touch(&mut self, target: Target) -> action::touch::TouchBuilder<'_> {
        action::touch::TouchBuilder::new(self, target)
    }

    /// Initiates a swipe action between two targets.
    ///
    /// Returns a `SwipeBuilder` to configure and execute the action.
    pub fn swipe(&mut self, start: Target, end: Target) -> action::swipe::SwipeBuilder<'_> {
        action::swipe::SwipeBuilder::new(self, start, end)
    }

    /// Waits for a target to appear on the screen.
    ///
    /// Returns a `WaitBuilder` to configure timeouts and execute the wait operation.
    /// The operation succeeds by returning the `Point` where the target was found.
    pub fn wait_for(&mut self, target: Target) -> action::wait::WaitBuilder<'_> {
        action::wait::WaitBuilder::new(self, target)
    }

    /// Initiates a text input action.
    ///
    /// Returns a `TextBuilder` to execute the action.
    pub fn text(&mut self, text: &str) -> action::text::TextBuilder<'_> {
        action::text::TextBuilder::new(self, text)
    }

    /// Pauses the script execution for a specified duration.
    pub fn sleep(&self, duration: Duration) {
        log::info!("Sleeping for {:?}", duration);
        std::thread::sleep(duration);
    }

    /// Takes a screenshot of the current device screen and returns it as an image object.
    ///
    /// This is the programmatic alternative to `snapshot`, which saves the image to a file.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `image::DynamicImage` on success.
    pub fn screenshot(&mut self) -> Result<DynamicImage> {
        self.controller.screenshot()
    }

    /// Takes a screenshot of the current device screen and saves it to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the screenshot image will be saved.
    pub fn snapshot<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path_ref = path.as_ref();
        log::info!("Saving snapshot to {:?}", path_ref);
        let image = self.screenshot()?;
        image.save(path_ref)?;
        Ok(())
    }

    /// Initiates a key event action (e.g., pressing Home or Back).
    ///
    /// Returns a `KeyeventBuilder` to configure and execute the action.
    pub fn keyevent(&mut self, key_code: KeyCode) -> action::keyevent::KeyeventBuilder<'_> {
        action::keyevent::KeyeventBuilder::new(self, key_code)
    }
}
