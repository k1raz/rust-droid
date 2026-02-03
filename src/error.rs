use std::path::PathBuf;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DroidError {
    #[error("ADB command failed: {0}")]
    AdbError(String),

    #[error("Device not found or specified serial is invalid")]
    DeviceNotFound,

    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Operation timed out after {0:?}")]
    Timeout(Duration),

    #[error("Could not find image target on screen: {0:?}")]
    ImageNotFound(PathBuf),

    #[error("Invalid target for operation: {0}")]
    InvalidTarget(String),

    #[error("GPU/OpenCL error: {0}")]
    GpuError(String),

    #[error("App package not found or has no launcher activity: {0}")]
    PackageNotFound(String),

    #[error("Failed to launch app '{package}': {output}")]
    AppLaunchFailed { package: String, output: String },
}

pub type Result<T> = std::result::Result<T, DroidError>;
