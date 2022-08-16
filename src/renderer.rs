use embedded_graphics_core::prelude::Point;

use crate::{fontreader::FontReader, Font};

pub const fn create_font_renderer<F: Font>() -> FontRenderer {
    FontRenderer::new::<F>()
}

pub struct FontRenderer {
    font: FontReader,
}

impl FontRenderer {
    pub(crate) const fn new<FONT: Font>() -> Self {
        Self {
            font: FontReader::new::<FONT>(),
        }
    }

    pub fn render_glyph<Color>(&self, ch: char, pos: Point, fg: Color, bg: Option<Color>) -> i32 {
        println!("{:#?}", self.font);
        todo!()
    }
}
