use crate::{Error, PixelRect, canvas, check_dimensions};
use image::{GenericImageView, Pixel, RgbaImage};

/// How to recognize empty margins; trimming crops only and never removes
/// background pixels inside the retained rectangle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrimMode {
    /// Use alpha when any pixel is non-opaque; otherwise trim white margins.
    #[default]
    Auto,
    /// Trim only pixels at or below the alpha threshold.
    Alpha,
    /// Trim transparent pixels and margins matching this RGB color.
    Color([u8; 3]),
    /// Keep the full input rectangle.
    None,
}

/// Empty-margin detection settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrimOptions {
    /// Background detection strategy.
    pub mode: TrimMode,
    /// Pixels with alpha at or below this value count as empty; defaults to 0.
    pub alpha_threshold: u8,
    /// Maximum per-channel difference from a solid background; defaults to 8.
    pub color_tolerance: u8,
}

impl Default for TrimOptions {
    fn default() -> Self {
        Self {
            mode: TrimMode::Auto,
            alpha_threshold: 0,
            color_tolerance: 8,
        }
    }
}

/// Cropped RGBA image and its original location.
#[derive(Debug, Clone)]
pub struct TrimmedImage {
    /// Retained pixels, without scaling or compositing.
    pub image: RgbaImage,
    /// Retained bounds in the source image.
    pub source_bounds: PixelRect,
}

/// Remove empty margins from any borrowed 8-bit image view.
///
/// Auto mode preserves white foregrounds on transparent inputs. Opaque inputs
/// use white margins; use [`TrimMode::Color`] for another plain background.
/// Complex backgrounds are retained, not segmented. A single matching color at
/// an edge never determines the background automatically.
///
/// # Errors
/// Rejects empty dimensions, images over the working-size limit, and inputs
/// containing only background pixels (except in [`TrimMode::None`]).
pub fn trim<I>(image: &I, options: &TrimOptions) -> Result<TrimmedImage, Error>
where
    I: GenericImageView + ?Sized,
    I::Pixel: Pixel<Subpixel = u8>,
{
    let (width, height) = image.dimensions();
    check_dimensions(width, height)?;
    let mode = match options.mode {
        TrimMode::Auto => {
            let has_alpha =
                (0..height).any(|y| (0..width).any(|x| image.get_pixel(x, y).to_rgba().0[3] < 255));
            if has_alpha {
                TrimMode::Alpha
            } else {
                TrimMode::Color([255, 255, 255])
            }
        }
        other => other,
    };
    let (mut left, mut top, mut right, mut bottom) = (width, height, 0, 0);
    for y in 0..height {
        for x in 0..width {
            let p = image.get_pixel(x, y).to_rgba().0;
            let visible = match mode {
                TrimMode::None => true,
                TrimMode::Alpha | TrimMode::Auto => p[3] > options.alpha_threshold,
                TrimMode::Color(background) => {
                    p[3] > options.alpha_threshold
                        && p[..3]
                            .iter()
                            .zip(background)
                            .any(|(channel, bg)| channel.abs_diff(bg) > options.color_tolerance)
                }
            };
            if visible {
                left = left.min(x);
                top = top.min(y);
                right = right.max(x + 1);
                bottom = bottom.max(y + 1);
            }
        }
    }
    if right <= left || bottom <= top {
        return Err(Error::EmptyImage);
    }
    let bounds = PixelRect {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    };
    let mut cropped = canvas(bounds.width, bounds.height)?;
    for (x, y, pixel) in cropped.enumerate_pixels_mut() {
        *pixel = image.get_pixel(left + x, top + y).to_rgba();
    }
    Ok(TrimmedImage {
        image: cropped,
        source_bounds: bounds,
    })
}
