use crate::{Droid, Result, models::KeyCode};
use std::time::Duration;

/// 用于构建和执行 "keyevent" (按键事件) 操作。
pub struct KeyeventBuilder<'a> {
    droid: &'a mut Droid,
    key_code: KeyCode,
    times: u32,
}

impl<'a> KeyeventBuilder<'a> {
    pub fn new(droid: &'a mut Droid, key_code: KeyCode) -> Self {
        Self {
            droid,
            key_code,
            times: 1,
        }
    }

    /// 设置按键次数。
    pub fn times(mut self, count: u32) -> Self {
        self.times = count;
        self
    }

    /// 执行按键操作。
    pub fn execute(self) -> Result<()> {
        log::info!(
            "Executing keyevent {:?} for {} times",
            self.key_code,
            self.times
        );

        for i in 0..self.times {
            self.droid.controller.input_keyevent(self.key_code as i32)?;

            if self.times > 1 && i < self.times - 1 {
                // 多次按键之间稍作停顿
                std::thread::sleep(Duration::from_millis(50));
            }
        }
        Ok(())
    }
}
