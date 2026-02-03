use super::rect::Rect;

#[derive(Debug, Clone, Copy)]
pub struct RelativeRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl RelativeRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        debug_assert!((0.0..=1.0).contains(&x));
        debug_assert!((0.0..=1.0).contains(&y));
        debug_assert!((0.0..=1.0).contains(&width));
        debug_assert!((0.0..=1.0).contains(&height));
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn to_absolute(&self, screen_width: u32, screen_height: u32) -> Rect {
        Rect {
            x: (self.x * screen_width as f32) as u32,
            y: (self.y * screen_height as f32) as u32,
            width: (self.width * screen_width as f32) as u32,
            height: (self.height * screen_height as f32) as u32,
        }
    }
}
