use rust_droid::{Droid, DroidConfig};

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("Connecting to device...");
    let config = DroidConfig::default();
    let mut droid = Droid::new(config)?;
    println!("Connection successful!");

    let snapshot_path = "test.png";
    println!("Taking a snapshot and saving to '{}'...", snapshot_path);
    droid.snapshot(snapshot_path)?;
    println!("Snapshot saved successfully.");

    println!("Pressing the back button...");
    droid
        .keyevent(rust_droid::models::KeyCode::Home)
        .execute()?;
    println!("Script finished successfully!");

    Ok(())
}
