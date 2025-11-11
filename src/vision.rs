use crate::common::rect::Rect as DroidRect;
use crate::error::{DroidError, Result};
use image::{DynamicImage, GenericImageView};
use imageproc::template_matching::{self, MatchTemplateMethod};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct MatchResult {
    pub rect: DroidRect,
    pub confidence: f32,
}

pub fn find_template(
    haystack: &DynamicImage,
    needle: &DynamicImage,
    threshold: f32,
    needle_path: &Path,
    search_rect: Option<DroidRect>,
) -> Result<MatchResult> {
    log::debug!(
        "Searching for template {:?} with threshold {:.2} inside region {:?}",
        needle_path,
        threshold,
        search_rect
    );

    let needle_gray = needle.to_luma8();
    let haystack_gray_full = haystack.to_luma8();

    let (haystack_to_search, offset_x, offset_y) = if let Some(rect) = search_rect {
        let cropped_view = haystack_gray_full.view(rect.x, rect.y, rect.width, rect.height);
        (cropped_view.to_image(), rect.x, rect.y)
    } else {
        (haystack_gray_full, 0, 0)
    };

    let result = template_matching::match_template_parallel(
        &haystack_to_search,
        &needle_gray,
        MatchTemplateMethod::CrossCorrelationNormalized,
    );

    let extremes = imageproc::template_matching::find_extremes(&result);
    let best_match_value = extremes.max_value;
    let mut best_match_location = extremes.max_value_location;

    best_match_location.0 += offset_x;
    best_match_location.1 += offset_y;

    log::trace!(
        "Best match found with confidence {:.4} at absolute point ({}, {})",
        best_match_value,
        best_match_location.0,
        best_match_location.1
    );

    if best_match_value >= threshold {
        let (needle_width, needle_height) = needle.dimensions();
        let result_rect = DroidRect::new(
            best_match_location.0,
            best_match_location.1,
            needle_width,
            needle_height,
        );

        let match_result = MatchResult {
            rect: result_rect,
            confidence: best_match_value,
        };

        log::debug!("Match found: {:?}", match_result);
        Ok(match_result)
    } else {
        log::warn!(
            "No match found for {:?}. Best confidence was {:.4}, which is below threshold {:.4}",
            needle_path,
            best_match_value,
            threshold
        );
        Err(DroidError::ImageNotFound(needle_path.to_path_buf()))
    }
}
