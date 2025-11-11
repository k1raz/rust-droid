# Rust Droid

[![Crates.io](https://img.shields.io/crates/v/rust-droid.svg)](https://crates.io/crates/rust-droid)
[![Docs.rs](https://docs.rs/rust-droid/badge.svg)](https://docs.rs/rust-droid)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/jhq223/rust-droid/actions/workflows/ci.yml/badge.svg)](https://github.com/jhq223/rust-droid/actions)

A powerful and fluent UI automation framework for Android, written in Rust. Inspired by popular tools like [Airtest](https://github.com/AirtestProject/Airtest) and `uiautomator2`.

`rust-droid` allows you to write simple, readable scripts to automate actions on Android devices using image recognition.

## Features

-   **Fluent Builder API**: Chain methods together to create readable and maintainable automation scripts.
-   **Image Recognition**: Find and interact with UI elements on the screen using template matching.
-   **Core Device Actions**: Supports tap, swipe, text input, key events, and screenshots.
-   **Robust Connection**: Automatically finds and connects to an available device via ADB.
-   **Cross-Platform**: Works on Windows, macOS, and Linux.

## Prerequisites

**USB Debugging**: Enable "USB debugging" in the "Developer options" on your Android device.

## Quick Start

Add `rust-droid` to your `Cargo.toml`:

```toml
[dependencies]
rust-droid = "0.1.0"
anyhow = "1.0"
env_logger = "0.11"
```

Here is a simple script that finds the "Settings" icon on the home screen, taps it, waits for the page to load, and takes a screenshot.

```rust
// examples/simple_script.rs
use rust_droid::{Droid, DroidConfig, Target, models::KeyCode};
use std::path::PathBuf;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // Initialize a logger to see internal logs from rust-droid.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 1. Create a Droid instance.
    // It will automatically connect to the first available device.
    println!("Connecting to device...");
    let mut droid = Droid::new(DroidConfig::default())?;
    println!("Connection successful!");

    // 2. Define the target image we want to find.
    let settings_icon_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/assets/settings_icon.png");
    let settings_icon = Target::Image(settings_icon_path);

    // 3. Wait for the settings icon to appear on the screen.
    // The `execute()` method returns the coordinates where the icon was found.
    println!("Waiting for settings icon...");
    let icon_position = droid.wait_for(settings_icon).timeout(Duration::from_secs(10)).execute()?;
    println!("Settings icon found at: {:?}", icon_position);

    // 4. Tap the icon. `icon_position` is a `Point`, which can be converted into a `Target`.
    println!("Tapping the settings icon...");
    droid.touch(icon_position.into()).execute()?;

    // 5. Wait for the settings page to load.
    droid.sleep(Duration::from_secs(2));

    // 6. Take a snapshot of the current screen.
    let snapshot_path = "settings_page.png";
    println!("Taking a snapshot and saving to '{}'...", snapshot_path);
    droid.snapshot(snapshot_path)?;
    println!("Snapshot saved successfully.");

    // 7. Press the back button to return to the home screen.
    println!("Pressing the back button...");
    droid.keyevent(KeyCode::Back).execute()?;
    println!("Script finished successfully!");

    Ok(())
}
```

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the [Apache-2.0 License](LICENSE).