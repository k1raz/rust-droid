use crate::common::point::Point;
use crate::common::relative_rect::RelativeRect;
use crate::{Droid, DroidError, Result, Target};
use std::time::{Duration, Instant};

pub struct WaitBuilder<'a> {
    droid: &'a mut Droid,
    target: Target,
    timeout: Duration,
    interval: Duration,
    threshold: Option<f32>,
    search_rect: Option<RelativeRect>,
}

impl<'a> WaitBuilder<'a> {
    pub fn new(droid: &'a mut Droid, target: Target) -> Self {
        let timeout = droid.config.default_timeout;
        let interval = droid.config.default_interval;
        Self {
            droid,
            target,
            timeout,
            interval,
            threshold: None,
            search_rect: None,
        }
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    pub fn interval(mut self, duration: Duration) -> Self {
        self.interval = duration;
        self
    }

    pub fn threshold(mut self, value: f32) -> Self {
        self.threshold = Some(value);
        self
    }

    pub fn search_in(mut self, rect: RelativeRect) -> Self {
        self.search_rect = Some(rect);
        self
    }

    pub fn execute(self) -> Result<Point> {
        let start_time = Instant::now();
        log::info!(
            "Waiting for target {:?} to appear, timeout: {:?}",
            self.target,
            self.timeout
        );

        let threshold = self
            .threshold
            .unwrap_or(self.droid.config.default_confidence);

        loop {
            if start_time.elapsed() > self.timeout {
                log::warn!("Wait operation timed out after {:?}", self.timeout);
                return Err(DroidError::Timeout(self.timeout));
            }

            match self
                .droid
                .resolve_target(&self.target, threshold, self.search_rect)
            {
                Ok(point) => {
                    log::info!("Target found at {:?}. Wait successful.", point);
                    return Ok(point);
                }
                Err(DroidError::ImageNotFound(_)) => {
                    log::trace!("Target not found yet, retrying after {:?}", self.interval);
                }
                Err(e) => {
                    log::error!("An unrecoverable error occurred while waiting: {:?}", e);
                    return Err(e);
                }
            }

            std::thread::sleep(self.interval);
        }
    }
}
