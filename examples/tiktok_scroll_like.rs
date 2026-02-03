use core::time::Duration;

use image::GenericImageView;
use rand::Rng;
use rust_droid::{AppPackages, Droid, DroidConfig, TIKTOK_LIKE_POINT, Target, error::DroidError};

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut droid = Droid::new(DroidConfig::default())?;

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

    droid.sleep(Duration::from_secs(3));

    let (w, h) = droid.screenshot()?.dimensions();
    let center_x = w / 2;
    let start_y = (h as f32 * 0.80) as u32;
    let end_y = (h as f32 * 0.20) as u32;

    let start = rust_droid::common::point::Point::new(center_x, start_y);
    let end = rust_droid::common::point::Point::new(center_x, end_y);

    let mut rng = rand::thread_rng();

    loop {
        let delay_secs = rng.gen_range(1..=15);
        droid.sleep(Duration::from_secs(delay_secs));

        droid
            .swipe(Target::Point(start), Target::Point(end))
            .duration(Duration::from_millis(350))
            .execute()?;

        droid.sleep(Duration::from_secs(3));
        droid.touch(Target::Point(TIKTOK_LIKE_POINT)).execute()?;
    }
}
