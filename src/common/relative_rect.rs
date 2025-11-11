// src/common/relative_rect.rs

use super::rect::Rect;

/// 使用相对坐标 (0.0 to 1.0) 定义一个矩形区域，以适应不同分辨率的设备。
#[derive(Debug, Clone, Copy)]
pub struct RelativeRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl RelativeRect {
    /// 创建一个新的相对矩形。
    ///
    /// # Panics
    /// 如果任何值不在 [0.0, 1.0] 范围内，将在 debug 模式下 panic。
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

    /// 将相对矩形转换为基于屏幕尺寸的绝对像素矩形。
    pub fn to_absolute(&self, screen_width: u32, screen_height: u32) -> Rect {
        Rect {
            x: (self.x * screen_width as f32) as u32,
            y: (self.y * screen_height as f32) as u32,
            width: (self.width * screen_width as f32) as u32,
            height: (self.height * screen_height as f32) as u32,
        }
    }
}
