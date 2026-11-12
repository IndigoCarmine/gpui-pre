// Added by ibis inc. for ibisOffice (2026-09-29): a record of what a window
// paints, for exporting it as vector graphics.

//! What a window painted, in order, as drawing commands rather than pixels:
//! for writing it out as vector graphics (a PDF page). Start recording with
//! [`crate::Window::record_paints`]; each frame starts a new record, and
//! [`crate::Window::take_recorded_paints`] gives the last frame's.
//!
//! Coordinates are in logical pixels from the window's top-left corner.
//! Colors already carry the element's opacity. `clip` is the content mask
//! the command was painted in.

use std::sync::Arc;

use crate::{
    Background, Bounds, BorderStyle, Corners, Edges, FontId, GlyphId, Hsla, PathOutline, Pixels,
    Point, RenderImage, TransformationMatrix,
};

/// One drawing command of a window.
#[derive(Clone, Debug)]
pub enum PaintRecord {
    /// A rectangle, maybe rounded, filled and bordered.
    Quad {
        /// The rectangle.
        bounds: Bounds<Pixels>,
        /// The content mask.
        clip: Bounds<Pixels>,
        /// The fill.
        background: Background,
        /// The border's color.
        border_color: Hsla,
        /// The border's widths, inside the rectangle.
        border_widths: Edges<Pixels>,
        /// The corners' radii.
        corner_radii: Corners<Pixels>,
        /// Solid or dashed border.
        border_style: BorderStyle,
    },
    /// A path: its outline when it has one, else the triangles it was
    /// tessellated into.
    Path {
        /// The outline, filled or stroked.
        outline: Option<Arc<PathOutline>>,
        /// The triangles, when there is no outline.
        triangles: Vec<[Point<Pixels>; 3]>,
        /// The fill.
        color: Background,
        /// The content mask.
        clip: Bounds<Pixels>,
    },
    /// A glyph of a font, its origin on the baseline.
    Glyph {
        /// Where the glyph's origin is.
        origin: Point<Pixels>,
        /// The font.
        font_id: FontId,
        /// The glyph in the font.
        glyph_id: GlyphId,
        /// The font's size.
        font_size: Pixels,
        /// The color (for an emoji, its own colors are used).
        color: Hsla,
        /// A transformation of the glyph, in scaled pixels about the
        /// window's origin (see [`crate::Window::paint_glyph_transformed`]).
        transformation: Option<TransformationMatrix>,
        /// A color glyph (an emoji).
        emoji: bool,
        /// The content mask.
        clip: Bounds<Pixels>,
    },
    /// A raster image.
    Image {
        /// Where the whole image is drawn.
        bounds: Bounds<Pixels>,
        /// The part shown, within the content mask.
        clip: Bounds<Pixels>,
        /// The corners' radii.
        corner_radii: Corners<Pixels>,
        /// The image (BGRA).
        image: Arc<RenderImage>,
        /// The frame of an animated image.
        frame_index: usize,
        /// Drawn in grays.
        grayscale: bool,
        /// The element's opacity.
        opacity: f32,
    },
    /// An underline or a strikethrough.
    Line {
        /// The line's rectangle.
        bounds: Bounds<Pixels>,
        /// The color.
        color: Hsla,
        /// A wavy line.
        wavy: bool,
        /// The content mask.
        clip: Bounds<Pixels>,
    },
    /// A shadow around (or inside) a rectangle.
    Shadow {
        /// The shadow's rectangle, before blurring.
        bounds: Bounds<Pixels>,
        /// The corners' radii.
        corner_radii: Corners<Pixels>,
        /// The color.
        color: Hsla,
        /// How far it is blurred.
        blur_radius: Pixels,
        /// Inside the rectangle.
        inset: bool,
        /// The element the shadow belongs to and its corners' radii: an inside shadow (`inset`) shows only inside it (modified by ibis inc.: `element` and `element_radii`).
        element: Bounds<Pixels>,
        /// The corner radii of `element`.
        element_radii: Corners<Pixels>,
        /// The content mask.
        clip: Bounds<Pixels>,
    },
}
