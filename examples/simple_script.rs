use rust_droid::{Droid, DroidConfig, Target};
use std::path::PathBuf;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // 初始化日志记录器，这样我们就能看到 rust-droid 内部的 log 信息
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();
    // 1. 创建 Droid 实例
    // 使用默认配置，它会自动连接到第一个找到的设备
    println!("Connecting to device...");
    let config = DroidConfig::default();
    let mut droid = Droid::new(config)?;
    println!("Connection successful!");

    // 2. 定义我们的目标图片
    // 使用 PathBuf 来构建跨平台的路径
    let settings_icon_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/assets/settings_icon.png");

    if !settings_icon_path.exists() {
        anyhow::bail!("Asset not found: {:?}", settings_icon_path);
    }

    let settings_icon = Target::Image(settings_icon_path);

    // 3. 等待设置图标出现
    let icon_position = droid.wait_for(settings_icon).execute()?;

    println!("Settings icon found at: {:?}", icon_position);

    // 4. 点击设置图标
    println!("Tapping the settings icon...");
    droid.touch(icon_position.into()).execute()?;

    // 5. 等待页面加载
    // 给一个短暂的延时，等待设置页面的开场动画完成
    println!("Waiting for settings page to load...");
    droid.sleep(Duration::from_secs(2));

    // 6. 截取当前屏幕
    let snapshot_path = "settings_page.png";
    println!("Taking a snapshot and saving to '{}'...", snapshot_path);
    droid.snapshot(snapshot_path)?;
    println!("Snapshot saved successfully.");

    // 7. 按下返回键，回到主屏幕
    println!("Pressing the back button...");
    droid
        .keyevent(rust_droid::models::KeyCode::Back)
        .execute()?;
    println!("Script finished successfully!");

    Ok(())
}
