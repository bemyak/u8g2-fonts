use embedded_graphics_core::{prelude::Point, primitives::Rectangle};

/// The vertical rendering position of the font.
///
/// Note that metrics like [`FontRenderer::get_glyph_bounding_box()`](crate::FontRenderer::get_glyph_bounding_box),
/// [`FontRenderer::get_ascent()`](crate::FontRenderer::get_ascent) or
/// [`FontRenderer::get_descent()`](crate::FontRenderer::get_descent)
/// are relative to [`FontPos::Baseline`].
///
/// The default is [`FontPos::Baseline`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontPos {
    /// Anchored at the font baseline
    Baseline,
    /// Anchored at the top
    Top,
    /// Anchored at the center
    Center,
    /// Anchored at the bottom
    Bottom,
}

impl Default for FontPos {
    fn default() -> Self {
        Self::Baseline
    }
}

/// The dimensions of a rendered glyph/text.
pub struct RenderedDimensions {
    /// The relative offset where a following glyph
    /// would have to get rendered.
    pub advance: Point,
    /// The bounding box of the rendered text/glyph.
    ///
    /// Can be `None` if nothing was rendered, like for
    /// a whitespace character.
    pub bounding_box: Option<Rectangle>,
}
