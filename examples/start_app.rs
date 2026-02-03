use core::time::Duration;

use rust_droid::{AppPackages, Droid, DroidConfig, error::DroidError};

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("Connecting to device...");
    let config = DroidConfig::default();
    let mut droid = Droid::new(config)?;
    println!("Connection successful!");

    match droid.launch_app_package(AppPackages::Tiktok) {
        Ok(()) => println!("ok"),
        Err(DroidError::PackageNotFound(pkg)) => {
            eprintln!("package not found: {}", pkg);
        }
        Err(DroidError::AppLaunchFailed { package, output }) => {
            eprintln!("doesnt started: {} -> {}", package, output);
        }
        Err(e) => eprintln!("any err: {e}"),
    }

    println!("Start tiktok");

    let snapshot_path = "open_tiktok.png";
    println!("Taking a snapshot and saving to '{}'...", snapshot_path);
    droid.snapshot(snapshot_path)?;
    println!("Snapshot saved successfully.");

    droid.sleep(Duration::from_secs(5));

    println!("Pressing the back button...");
    droid
        .keyevent(rust_droid::models::KeyCode::Home)
        .execute()?;
    println!("Script finished successfully!");

    Ok(())
}
