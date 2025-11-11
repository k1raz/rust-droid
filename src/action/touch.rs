use crate::common::relative_rect::RelativeRect;
use crate::{Droid, Result, Target};
use std::time::Duration;

/// Builds and executes a "touch" action.
///
/// A touch can be a simple tap or a long press, depending on the `duration`.
/// This struct is created by the `Droid::touch()` method.
pub struct TouchBuilder<'a> {
    droid: &'a mut Droid,
    target: Target,
    duration: Duration,
    times: u32,
    threshold: Option<f32>,
    search_rect: Option<RelativeRect>,
}

impl<'a> TouchBuilder<'a> {
    pub fn new(droid: &'a mut Droid, target: Target) -> Self {
        Self {
            droid,
            target,
            duration: Duration::from_millis(100),
            times: 1,
            threshold: None,
            search_rect: None,
        }
    }

    /// Sets the number of times to perform the touch action.
    ///
    /// Default is `1`.
    pub fn times(mut self, count: u32) -> Self {
        self.times = count;
        self
    }

    /// Sets the duration of the touch.
    ///
    /// A short duration (e.g., < 200ms) is a tap.
    /// A longer duration is a long press.
    ///
    /// Default is `100ms`.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the confidence threshold for image matching.
    ///
    /// This only applies if the `target` is an `Image`.
    /// The value should be between `0.0` and `1.0`.
    /// If not set, the default confidence from `DroidConfig` is used.
    pub fn threshold(mut self, value: f32) -> Self {
        self.threshold = Some(value);
        self
    }

    /// Restricts the image search to a specific region of the screen.
    ///
    /// This only applies if the `target` is an `Image`.
    /// The `rect` is defined using relative coordinates (0.0 to 1.0).
    pub fn search_in(mut self, rect: RelativeRect) -> Self {
        self.search_rect = Some(rect);
        self
    }

    /// Executes the configured touch action.
    ///
    /// It resolves the `target` to a screen coordinate and then performs
    /// the tap or long press.
    ///
    /// # Errors
    ///
    /// Returns an error if the target cannot be found (for image targets) or
    /// if the underlying ADB command fails.
    pub fn execute(self) -> Result<()> {
        let threshold = self
            .threshold
            .unwrap_or(self.droid.config.default_confidence);
        let point = self
            .droid
            .resolve_target(&self.target, threshold, self.search_rect)?;
        log::info!(
            "Executing touch action at {:?} for {} times",
            point,
            self.times
        );

        for i in 0..self.times {
            if self.duration <= Duration::from_millis(200) {
                self.droid.controller.tap(point)?;
            } else {
                self.droid.controller.swipe(point, point, self.duration)?;
            }

            if self.times > 1 && i < self.times - 1 {
                std::thread::sleep(Duration::from_millis(50));
            }
        }
        Ok(())
    }
}
