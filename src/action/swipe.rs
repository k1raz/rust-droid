use crate::common::relative_rect::RelativeRect;
use crate::{Droid, Result, Target};
use std::time::Duration;

pub struct SwipeBuilder<'a> {
    droid: &'a mut Droid,
    start: Target,
    end: Target,
    duration: Duration,
    threshold: Option<f32>,
    start_search_rect: Option<RelativeRect>,
    end_search_rect: Option<RelativeRect>,
}

impl<'a> SwipeBuilder<'a> {
    pub fn new(droid: &'a mut Droid, start: Target, end: Target) -> Self {
        Self {
            droid,
            start,
            end,
            duration: Duration::from_millis(300),
            threshold: None,
            start_search_rect: None,
            end_search_rect: None,
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn threshold(mut self, value: f32) -> Self {
        self.threshold = Some(value);
        self
    }

    pub fn search_start_in(mut self, rect: RelativeRect) -> Self {
        self.start_search_rect = Some(rect);
        self
    }

    pub fn search_end_in(mut self, rect: RelativeRect) -> Self {
        self.end_search_rect = Some(rect);
        self
    }

    pub fn execute(self) -> Result<()> {
        let threshold = self
            .threshold
            .unwrap_or(self.droid.config.default_confidence);
        let start_point =
            self.droid
                .resolve_target(&self.start, threshold, self.start_search_rect)?;
        let end_point = self
            .droid
            .resolve_target(&self.end, threshold, self.end_search_rect)?;

        log::info!(
            "Executing swipe from {:?} to {:?} over {:?}",
            start_point,
            end_point,
            self.duration
        );

        self.droid
            .controller
            .swipe(start_point, end_point, self.duration)?;

        Ok(())
    }
}
