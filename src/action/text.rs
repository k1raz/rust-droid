use crate::{Droid, Result};

/// 用于构建和执行 "text" (输入文本) 操作。
pub struct TextBuilder<'a> {
    droid: &'a mut Droid,
    text: String,
}

impl<'a> TextBuilder<'a> {
    pub fn new(droid: &'a mut Droid, text: &str) -> Self {
        Self {
            droid,
            text: text.to_string(),
        }
    }

    /// 执行文本输入操作。
    pub fn execute(self) -> Result<()> {
        log::info!("Executing text input: '{}'", self.text);
        self.droid.controller.input_text(&self.text)
    }
}
