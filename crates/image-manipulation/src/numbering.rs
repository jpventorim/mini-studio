use ab_glyph::{Font, FontRef, ScaleFont, point};
use image::Rgba;

use crate::{Error, PreparedImage};

// Bundled with its SIL Open Font License; no system font installation needed.
const FONT: &[u8] = include_bytes!("../assets/fonts/AtkinsonHyperlegible-Bold.ttf");

pub(crate) fn draw(output: &mut PreparedImage, number: usize) -> Result<(), Error> {
    let font = FontRef::try_from_slice(FONT)
        .map_err(|_| Error::InvalidInput("cannot read the bundled numbering font"))?;
    let text = number.to_string();
    let ppm = output.dpi / 25.4;
    let standard_inset = ppm.ceil() as u32;
    let base = output.base_bounds;
    let numbered_depth = if output.base_has_flap {
        base.height / 2
    } else {
        base.height
    };
    // Quarter-size sections cannot hold a 2.5 mm digit plus two 1 mm insets.
    // Adapt only the label inset; the artwork's cutting space is unchanged.
    let compact =
        numbered_depth.saturating_sub(2 * standard_inset + 1) < (2.5 * ppm).ceil() as u32 + 1;
    let inset = if compact {
        standard_inset
            .min((f64::from(numbered_depth) * 0.1).ceil() as u32)
            .max(1)
    } else {
        standard_inset
    };
    // Exclude the join and midpoint strokes, retaining space on every side.
    let area_width = base.width.saturating_sub(2 * inset);
    let area_height = numbered_depth.saturating_sub(2 * inset + 1);
    let too_small = || {
        Error::InvalidInput(
            "copy number is too small to render clearly; increase the base size or DPI",
        )
    };
    if area_width <= 2 || area_height <= 2 {
        return Err(too_small());
    }

    // Measure actual outlines, rather than the font's larger line-height box.
    let mut cursor = 0.0_f32;
    let mut left = f32::INFINITY;
    let mut right = f32::NEG_INFINITY;
    let mut bottom = f32::INFINITY;
    let mut top = f32::NEG_INFINITY;
    for ch in text.chars() {
        let id = font.glyph_id(ch);
        let outline = font.outline(id).ok_or_else(too_small)?;
        left = left.min(cursor + outline.bounds.min.x);
        right = right.max(cursor + outline.bounds.max.x);
        // Font outlines use an upward Y axis: min.y is the top bound.
        bottom = bottom.min(outline.bounds.max.y);
        top = top.max(outline.bounds.min.y);
        cursor += font.h_advance_unscaled(id);
    }
    let height = top - bottom;
    let factor = ((area_width - 1) as f32 / (right - left)).min((area_height - 1) as f32 / height);
    let digit_height = height * factor;
    if digit_height < 8.0 || (!compact && digit_height < (2.5 * ppm) as f32) {
        return Err(too_small());
    }
    let scaled = font.as_scaled(factor * font.height_unscaled());
    let mut cursor = 0.0;
    let mut glyphs = Vec::with_capacity(text.len());
    for ch in text.chars() {
        let id = scaled.glyph_id(ch);
        let glyph = id.with_scale_and_position(scaled.scale(), point(cursor, 0.0));
        glyphs.push(font.outline_glyph(glyph).ok_or_else(too_small)?);
        cursor += scaled.h_advance(id);
    }
    let min_x = glyphs
        .iter()
        .map(|g| g.px_bounds().min.x as i32)
        .min()
        .ok_or_else(too_small)?;
    let min_y = glyphs
        .iter()
        .map(|g| g.px_bounds().min.y as i32)
        .min()
        .ok_or_else(too_small)?;
    let max_x = glyphs
        .iter()
        .map(|g| g.px_bounds().max.x as i32)
        .max()
        .ok_or_else(too_small)?;
    let max_y = glyphs
        .iter()
        .map(|g| g.px_bounds().max.y as i32)
        .max()
        .ok_or_else(too_small)?;
    let width = (max_x - min_x) as u32;
    let height = (max_y - min_y) as u32;
    if width > area_width || height > area_height {
        return Err(too_small());
    }
    let x = base.x + inset + (area_width - width) / 2;
    let y = base.y + 1 + inset + (area_height - height) / 2;
    for glyph in glyphs {
        let bounds = glyph.px_bounds();
        let offset_x = (bounds.min.x as i32 - min_x) as u32;
        let offset_y = (bounds.min.y as i32 - min_y) as u32;
        glyph.draw(|gx, gy, coverage| {
            let px = x + offset_x + gx;
            let py = y + offset_y + gy;
            let old = output.image.get_pixel(px, py)[0];
            let gray = (f32::from(old) * (1.0 - coverage.clamp(0.0, 1.0))).round() as u8;
            let pixel = Rgba([gray, gray, gray, 255]);
            output.image.put_pixel(px, py, pixel);
            output
                .image
                .put_pixel(px, output.image.height() - 1 - py, pixel);
        });
    }
    output.copy_number = Some(number);
    Ok(())
}
