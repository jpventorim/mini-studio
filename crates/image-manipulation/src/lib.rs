//! Prepare printable, foldable standees from borrowed images.
//!
//! Trims transparent or plain-background margins, scales the artwork to 90% of
//! the base width, adds a rectangular base tab, and mirrors the completed half above
//! the original. Each artwork panel prefers 1.5 times the base width, including
//! padding, and shortens to fit the height limit while preserving base width.
//! Position places artwork on the ground, hovering, or flying. The complete
//! strip defaults to a 297 mm height limit. Below size 4, each base has a
//! halfway line; size 4 and up keep only the inner, numbered section.
//!
//! ```
//! use image::{Rgba, RgbaImage};
//! use image_manipulation::{prepare, ManipulationOptions};
//! let source = RgbaImage::from_pixel(20, 40, Rgba([80, 30, 140, 255]));
//! let output = prepare(&source, &ManipulationOptions::default())?;
//! assert_eq!(output.width_mm, 23.0);
//! assert_eq!(output.image.height() % 2, 0);
//! # Ok::<(), image_manipulation::Error>(())
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod numbering;
mod trim;
pub use trim::{TrimMode, TrimOptions, TrimmedImage, trim};

use image::{GenericImageView, Pixel, Rgba, RgbaImage, imageops};

/// Maximum pixels in any working image, to bound memory usage.
pub const MAX_IMAGE_PIXELS: u64 = 16_777_216;

/// Rectangle in source or output pixels, with a top-left origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelRect {
    /// Left edge.
    pub x: u32,
    /// Top edge.
    pub y: u32,
    /// Width.
    pub width: u32,
    /// Height.
    pub height: u32,
}

/// Shape of the base added below the artwork before mirroring.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum BaseShape {
    /// A tab sized so the two opposite folded feet span the grid-sized width.
    /// Below size 4, its outer half folds underneath. At size 4 and up it
    /// contains only one section, half the footprint depth.
    #[default]
    Rectangle,
}

/// Vertical position of the artwork inside its height-limited panel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Position {
    /// Bottom-align artwork with 1 mm of clean space above the base.
    #[default]
    Grounded,
    /// Raise artwork by 10% of panel height above its grounded position.
    Hovering,
    /// Raise artwork by 30% of panel height above its grounded position.
    Flying,
}

impl Position {
    fn elevation_ratio(self) -> f64 {
        match self {
            Self::Grounded => 0.0,
            Self::Hovering => 0.1,
            Self::Flying => 0.3,
        }
    }
}

/// Physical sizing and trimming options.
#[derive(Debug, Clone, PartialEq)]
pub struct ManipulationOptions {
    /// Black outline outside the completed strip, in pixels; defaults to zero.
    /// The outline increases physical output dimensions without shrinking artwork.
    pub border_px: u32,
    /// Number copies from 1 for each source; defaults to false.
    /// Use [`prepare_copies`] for quantities. [`prepare`] labels a single copy 1.
    pub number_copies: bool,
    /// Grid multiplier: 1 means a nominal 25 mm footprint with the default grid.
    pub size: f64,
    /// Millimetres per grid unit; defaults to 25.
    pub grid_size_mm: f64,
    /// Optional total clearance subtracted from the nominal footprint.
    /// `None` uses 1 mm total for sizes below 1 and 2 mm total for sizes 1 and up.
    /// An explicit value overrides this rule in both base dimensions.
    pub base_clearance_mm: Option<f64>,
    /// Target raster resolution; defaults to 300 pixels per inch.
    pub dpi: f64,
    /// Base shape; a rectangle sized to preserve the folded grid footprint.
    pub base_shape: BaseShape,
    /// Vertical placement of artwork; defaults to grounded.
    pub position: Position,
    /// Fraction of the base width occupied by artwork; defaults to 0.9.
    pub artwork_width_ratio: f64,
    /// Maximum height of the complete mirrored strip, including both base tabs.
    /// Defaults to 297 mm. Artwork panels shorten to fit without reducing base
    /// width. Returns an error when base tabs and minimum insets cannot fit.
    pub max_height_mm: f64,
    /// How to identify empty source margins.
    pub trim: TrimOptions,
}

impl Default for ManipulationOptions {
    fn default() -> Self {
        Self {
            border_px: 0,
            number_copies: false,
            size: 1.0,
            grid_size_mm: 25.0,
            base_clearance_mm: None,
            dpi: 300.0,
            base_shape: BaseShape::Rectangle,
            position: Position::Grounded,
            artwork_width_ratio: 0.9,
            max_height_mm: 297.0,
            trim: TrimOptions::default(),
        }
    }
}

/// Prepared RGBA pixels and physical dimensions suitable for `image-packing`.
#[derive(Debug, Clone)]
pub struct PreparedImage {
    /// Whether each base retains its outer unnumbered flap; false at size 4 and up.
    pub base_has_flap: bool,
    /// Number printed on both base tabs, or `None` when numbering is disabled.
    pub copy_number: Option<usize>,
    /// Mirrored pair: flipped base and artwork above upright artwork and base.
    pub image: RgbaImage,
    /// Total output width, including any border, in millimetres.
    pub width_mm: f64,
    /// Total height at the same physical scale, in millimetres.
    pub height_mm: f64,
    /// Effective DPI after rounding the base width to whole pixels.
    pub dpi: f64,
    /// Bounds retained from the original input.
    pub source_bounds: PixelRect,
    /// Upright artwork rectangle in the lower half, excluding its padding.
    pub artwork_bounds: PixelRect,
    /// Upright base rectangle, at the bottom of the strip.
    pub base_bounds: PixelRect,
    /// Horizontal fold coordinate between the mirrored halves.
    pub fold_y: u32,
}

/// Validation, memory allocation, or empty-image failures.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Invalid physical size, resolution, ratio, or empty dimensions.
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
    /// No pixels remain after trimming the background.
    #[error("the image contains no foreground pixels after trimming")]
    EmptyImage,
    /// Base tabs and minimum artwork insets exceed the available paper height.
    #[error(
        "base tabs and cutting insets require at least {required_mm:.2} mm height, exceeding the {limit_mm:.2} mm limit"
    )]
    HeightLimitExceeded {
        /// Height of the full strip, including both tabs.
        required_mm: f64,
        /// Configured available height.
        limit_mm: f64,
    },
    /// Dimensions exceed arithmetic or working-memory limits.
    #[error("image dimensions exceed the supported limit of {MAX_IMAGE_PIXELS} pixels")]
    ImageTooLarge,
    /// A working image could not be allocated.
    #[error("cannot allocate image buffer: {0}")]
    Allocation(#[from] std::collections::TryReserveError),
}

pub(crate) fn check_dimensions(width: u32, height: u32) -> Result<(), Error> {
    if width == 0 || height == 0 {
        return Err(Error::InvalidInput("image dimensions must be positive"));
    }
    if u64::from(width) * u64::from(height) > MAX_IMAGE_PIXELS {
        return Err(Error::ImageTooLarge);
    }
    Ok(())
}

pub(crate) fn canvas(width: u32, height: u32) -> Result<RgbaImage, Error> {
    check_dimensions(width, height)?;
    let count = usize::try_from(u64::from(width) * u64::from(height) * 4)
        .map_err(|_| Error::ImageTooLarge)?;
    let mut pixels = Vec::new();
    pixels.try_reserve_exact(count)?;
    pixels.resize(count, 0);
    RgbaImage::from_raw(width, height, pixels).ok_or(Error::ImageTooLarge)
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn dimension(value: f64) -> Result<u32, Error> {
    if !value.is_finite() || value.round() > f64::from(u32::MAX) {
        return Err(Error::ImageTooLarge);
    }
    if value.round() < 1.0 {
        return Err(Error::InvalidInput(
            "physical dimensions must span at least one pixel",
        ));
    }
    Ok(value.round() as u32)
}

// Premultiplication prevents hidden RGB in transparent pixels producing fringes.
fn resize_artwork(source: &RgbaImage, width: u32, height: u32) -> Result<RgbaImage, Error> {
    check_dimensions(width, height)?;
    let premultiplied = image::Rgba32FImage::from_fn(source.width(), source.height(), |x, y| {
        let p = source.get_pixel(x, y).0;
        let a = f32::from(p[3]) / 255.0;
        image::Rgba([
            f32::from(p[0]) / 255.0 * a,
            f32::from(p[1]) / 255.0 * a,
            f32::from(p[2]) / 255.0 * a,
            a,
        ])
    });
    let resized = imageops::resize(
        &premultiplied,
        width,
        height,
        imageops::FilterType::Lanczos3,
    );
    let mut output = canvas(width, height)?;
    for (pixel, sample) in output.pixels_mut().zip(resized.pixels()) {
        let alpha = sample.0[3].clamp(0.0, 1.0);
        if alpha > 0.0 {
            *pixel = Rgba([
                ((sample.0[0] / alpha).clamp(0.0, 1.0) * 255.0).round() as u8,
                ((sample.0[1] / alpha).clamp(0.0, 1.0) * 255.0).round() as u8,
                ((sample.0[2] / alpha).clamp(0.0, 1.0) * 255.0).round() as u8,
                (alpha * 255.0).round() as u8,
            ]);
        }
    }
    Ok(output)
}

/// Trim, resize, add a base, and create a vertically mirrored foldable pair.
///
/// Size multiplies grid width, then the explicit or automatic clearance is subtracted to obtain
/// the physical base width. Artwork fits within 90% of that width by default,
/// with a padded panel up to 1.5 times the base width in height. Both artwork dimensions
/// scale proportionally. Artwork is centered horizontally with at least 1 mm
/// of clear space on every side, excluding the fold line. Insets round up to
/// whole pixels at the effective physical scale. Position reserves elevation
/// above the base before fitting artwork. Two folded feet together span the grid-sized base width.
/// Artwork panels shorten to fit `max_height_mm`; base width and tab depth stay unchanged.
/// Black one-pixel horizontal lines mark the artwork join and center fold,
/// plus a base midpoint below size 4. The complete upper half is a vertical reflection of the
/// lower half; no source pixels are cropped after trimming.
///
/// # Errors
/// Rejects non-positive/non-finite sizes, DPI, or height limits, ratios outside
/// `(0, 1]`, base tabs and minimum insets exceeding the height limit, insufficient space for
/// padded artwork and elevation, subpixel
/// dimensions, empty foregrounds, and images over [`MAX_IMAGE_PIXELS`].
pub fn prepare<I>(image: &I, options: &ManipulationOptions) -> Result<PreparedImage, Error>
where
    I: GenericImageView + ?Sized,
    I::Pixel: Pixel<Subpixel = u8>,
{
    if !positive(options.size)
        || !positive(options.grid_size_mm)
        || !positive(options.dpi)
        || !positive(options.max_height_mm)
    {
        return Err(Error::InvalidInput(
            "size, grid size, dpi, and maximum height must be finite and positive",
        ));
    }
    if !positive(options.artwork_width_ratio) || options.artwork_width_ratio > 1.0 {
        return Err(Error::InvalidInput("artwork width ratio must be in (0, 1]"));
    }
    let clearance = options
        .base_clearance_mm
        .unwrap_or(if options.size < 1.0 { 1.0 } else { 2.0 });
    if !clearance.is_finite() || clearance < 0.0 {
        return Err(Error::InvalidInput(
            "base clearance must be finite and non-negative",
        ));
    }
    let width_mm = options.size * options.grid_size_mm - clearance;
    if width_mm <= 0.0 {
        return Err(Error::InvalidInput(
            "base clearance must be smaller than the nominal footprint",
        ));
    }
    let base_width = dimension(width_mm * options.dpi / 25.4)?;
    let pixels_per_mm = f64::from(base_width) / width_mm;
    let base_has_flap = options.size < 4.0;
    // The two folded feet together span the grid-sized base width. Smaller
    // tabs include a second section folded underneath; larger tabs do not.
    let base_depth = match options.base_shape {
        BaseShape::Rectangle if base_has_flap => base_width,
        BaseShape::Rectangle => (base_width / 2).max(1),
    };
    let padding = dimension(pixels_per_mm.ceil())?;
    let horizontal_padding = padding.checked_mul(2).ok_or(Error::ImageTooLarge)?;
    // The center fold consumes one pixel in each half, outside the clear inset.
    let vertical_padding = horizontal_padding
        .checked_add(1)
        .ok_or(Error::ImageTooLarge)?;
    let inner_width = base_width
        .checked_sub(horizontal_padding)
        .filter(|width| *width > 0)
        .ok_or(Error::InvalidInput(
            "base width leaves no room for 1 mm insets and artwork",
        ))?;
    let max_art_width =
        dimension(f64::from(base_width) * options.artwork_width_ratio)?.min(inner_width);
    let border_twice = options
        .border_px
        .checked_mul(2)
        .ok_or(Error::ImageTooLarge)?;
    let minimum_panel = dimension(
        ((f64::from(vertical_padding) + 1.0) / (1.0 - options.position.elevation_ratio())).ceil(),
    )?;
    let minimum_half = base_depth
        .checked_add(minimum_panel)
        .ok_or(Error::ImageTooLarge)?;
    let minimum_height = minimum_half
        .checked_mul(2)
        .and_then(|height| height.checked_add(border_twice))
        .ok_or(Error::ImageTooLarge)?;
    let available_pixels = (options.max_height_mm * f64::from(base_width) / width_mm)
        .floor()
        .min(f64::from(u32::MAX)) as u32;
    if available_pixels < minimum_height {
        return Err(Error::HeightLimitExceeded {
            required_mm: f64::from(minimum_height) * width_mm / f64::from(base_width),
            limit_mm: options.max_height_mm,
        });
    }
    let preferred_panel = dimension((f64::from(base_width) * 1.5).floor())?;
    let available_half = (available_pixels - border_twice) / 2;
    let panel_height = preferred_panel.min(available_half - base_depth);
    let half_height = panel_height
        .checked_add(base_depth)
        .ok_or(Error::ImageTooLarge)?;
    let total_height = half_height.checked_mul(2).ok_or(Error::ImageTooLarge)?;
    let output_width = base_width
        .checked_add(border_twice)
        .ok_or(Error::ImageTooLarge)?;
    let output_height = total_height
        .checked_add(border_twice)
        .ok_or(Error::ImageTooLarge)?;
    check_dimensions(output_width, output_height)?;
    let height_mm = f64::from(output_height) * width_mm / f64::from(base_width);
    if height_mm > options.max_height_mm {
        return Err(Error::HeightLimitExceeded {
            required_mm: height_mm,
            limit_mm: options.max_height_mm,
        });
    }
    check_dimensions(base_width, total_height)?;
    // The ratio is bounded to [0, 0.3], so this conversion fits in u32.
    let elevation = (f64::from(panel_height) * options.position.elevation_ratio()).ceil() as u32;
    let max_art_height = panel_height
        .checked_sub(vertical_padding)
        .and_then(|height| height.checked_sub(elevation))
        .filter(|height| *height > 0)
        .ok_or(Error::InvalidInput(
            "panel leaves no room for cutting insets, position, and artwork",
        ))?;
    let trimmed = trim(image, &options.trim)?;
    let scale = (f64::from(max_art_width) / f64::from(trimmed.image.width()))
        .min(f64::from(max_art_height) / f64::from(trimmed.image.height()));
    let art_width = dimension(f64::from(trimmed.image.width()) * scale)?.min(max_art_width);
    let art_height = dimension(f64::from(trimmed.image.height()) * scale)?.min(max_art_height);
    let art_x = (base_width - art_width) / 2;
    let art_y = panel_height - padding - elevation - art_height;
    let artwork = resize_artwork(&trimmed.image, art_width, art_height)?;
    let mut half = canvas(base_width, half_height)?;
    imageops::replace(&mut half, &artwork, i64::from(art_x), i64::from(art_y));
    for y in panel_height..half_height {
        for x in 0..base_width {
            half.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }
    for y in
        std::iter::once(panel_height).chain(base_has_flap.then_some(panel_height + base_depth / 2))
    {
        for x in 0..base_width {
            half.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        }
    }
    // Mirror the completed half, including its tab and divider lines.
    let mut output = canvas(base_width, total_height)?;
    for y in 0..half_height {
        for x in 0..base_width {
            let p = *half.get_pixel(x, y);
            output.put_pixel(x, half_height + y, p);
            output.put_pixel(x, half_height - 1 - y, p);
        }
    }
    // A symmetric two-row center mark keeps the pair exactly mirrored.
    for y in [half_height - 1, half_height] {
        for x in 0..base_width {
            output.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        }
    }
    let mut result = PreparedImage {
        base_has_flap,
        copy_number: None,
        width_mm,
        height_mm,
        dpi: f64::from(base_width) * 25.4 / width_mm,
        source_bounds: trimmed.source_bounds,
        artwork_bounds: PixelRect {
            x: art_x,
            y: half_height + art_y,
            width: art_width,
            height: art_height,
        },
        base_bounds: PixelRect {
            x: 0,
            y: half_height + panel_height,
            width: base_width,
            height: base_depth,
        },
        fold_y: half_height,
        image: output,
    };
    if options.number_copies {
        numbering::draw(&mut result, 1)?;
    }
    if options.border_px > 0 {
        let mut bordered = canvas(output_width, output_height)?;
        for pixel in bordered.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 255]);
        }
        imageops::replace(
            &mut bordered,
            &result.image,
            i64::from(options.border_px),
            i64::from(options.border_px),
        );
        result.image = bordered;
        result.width_mm = f64::from(output_width) * width_mm / f64::from(base_width);
        result.artwork_bounds.x += options.border_px;
        result.artwork_bounds.y += options.border_px;
        result.base_bounds.x += options.border_px;
        result.base_bounds.y += options.border_px;
        result.fold_y += options.border_px;
    }
    Ok(result)
}

/// Copies of one prepared source, generated one at a time to bound memory use.
///
/// Each call to [`prepare_copies`] starts numbering at 1. Items return an error
/// if a number cannot fit at a legible resolution. Number insets adapt to small
/// sections; the artwork's 1 mm cutting inset is unchanged.
pub struct PreparedCopies {
    source: PreparedImage,
    quantity: usize,
    next: usize,
    numbered: bool,
}

impl Iterator for PreparedCopies {
    type Item = Result<PreparedImage, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.quantity {
            return None;
        }
        self.next += 1;
        let mut copy = self.source.clone();
        if self.numbered
            && let Err(error) = numbering::draw(&mut copy, self.next)
        {
            return Some(Err(error));
        }
        Some(Ok(copy))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.quantity - self.next;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for PreparedCopies {}
impl std::iter::FusedIterator for PreparedCopies {}

/// Prepare one source once, then yield its copies with independent numbering.
///
/// Numbers appear in the inner half of each base tab. Numbering restarts at 1
/// on every call, even if another source was prepared previously.
///
/// # Examples
/// ```
/// use image::{Rgba, RgbaImage};
/// use image_manipulation::{ManipulationOptions, prepare_copies};
/// let source = RgbaImage::from_pixel(10, 20, Rgba([80, 20, 150, 255]));
/// let options = ManipulationOptions { number_copies: true, ..Default::default() };
/// for (index, copy) in prepare_copies(&source, 4, &options)?.enumerate() {
///     assert_eq!(copy?.copy_number, Some(index + 1));
/// }
/// # Ok::<(), image_manipulation::Error>(())
/// ```
///
/// # Errors
/// Returns the same preparation errors as [`prepare`], or rejects zero copies.
/// Individual copies can also fail if their number cannot fit legibly.
pub fn prepare_copies<I>(
    image: &I,
    quantity: usize,
    options: &ManipulationOptions,
) -> Result<PreparedCopies, Error>
where
    I: GenericImageView + ?Sized,
    I::Pixel: Pixel<Subpixel = u8>,
{
    if quantity == 0 {
        return Err(Error::InvalidInput("quantity must be positive"));
    }
    let mut unnumbered = options.clone();
    unnumbered.number_copies = false;
    Ok(PreparedCopies {
        source: prepare(image, &unnumbered)?,
        quantity,
        next: 0,
        numbered: options.number_copies,
    })
}
