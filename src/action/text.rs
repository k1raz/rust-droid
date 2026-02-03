use crate::{Droid, Result};

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

    pub fn execute(self) -> Result<()> {
        log::info!("Executing text input: '{}'", self.text);
        self.droid.controller.input_text(&self.text)
    }
}
