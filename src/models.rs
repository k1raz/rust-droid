use crate::common::point::Point;
use std::path::{Path, PathBuf};

/// Represents the target of an operation, which can be a precise coordinate
/// or an image that needs to be located on the screen.
#[derive(Debug, Clone)]
pub enum Target {
    /// An absolute coordinate on the screen.
    Point(Point),
    /// The path to an image file to be used as a template for visual search.
    Image(PathBuf),
}

impl From<Point> for Target {
    fn from(point: Point) -> Self {
        Target::Point(point)
    }
}

impl From<PathBuf> for Target {
    fn from(path: PathBuf) -> Self {
        Target::Image(path)
    }
}

impl From<&Path> for Target {
    fn from(path: &Path) -> Self {
        Target::Image(path.to_path_buf())
    }
}

impl From<&str> for Target {
    fn from(path: &str) -> Self {
        Target::Image(PathBuf::from(path))
    }
}

/// An enumeration of common Android key codes.
///
/// These are sent to the device using `input keyevent`.
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum KeyCode {
    Home = 3,
    Back = 4,
    Enter = 66,
}
