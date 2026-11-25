use std::sync::Arc;

use image::{GenericImageView, Pixel};
use krilla::{
    Document, SerializeSettings,
    geom::{Size, Transform},
    image::{BitsPerComponent, CustomImage, Image, ImageColorspace},
    page::PageSettings,
};

use crate::{DocumentLayout, Error, ImageItem};

// Unlike Image::from_rgba8 in Krilla 0.8, include the dimensions in the hash.
// Cloning shares channels, and Krilla computes the content hash once at creation.
#[derive(Clone, Hash)]
struct Bitmap {
    size: (u32, u32),
    rgb: Arc<Vec<u8>>,
    bytes_per_component: u8,
    alpha: Arc<Vec<u8>>,
}

impl CustomImage for Bitmap {
    fn color_channel(&self) -> &[u8] {
        &self.rgb
    }
    fn alpha_channel(&self) -> Option<&[u8]> {
        Some(&self.alpha)
    }
    fn bits_per_component(&self) -> BitsPerComponent {
        if self.bytes_per_component == 2 {
            BitsPerComponent::Sixteen
        } else {
            BitsPerComponent::Eight
        }
    }
    fn size(&self) -> (u32, u32) {
        self.size
    }
    fn icc_profile(&self) -> Option<&[u8]> {
        None
    }
    fn color_space(&self) -> ImageColorspace {
        ImageColorspace::Rgb
    }
}

/// Print details saved in the PDF. Page and image sizes control printing.
#[derive(Debug, Clone, Default)]
pub struct PdfMetadata {
    /// Title shown in PDF Properties and saved in XMP metadata.
    pub title: String,
    /// Short print summary shown in PDF Properties and saved in XMP metadata.
    pub description: String,
    /// Settings saved in the `ExportSettings` field for checking the export.
    /// Printer software does not use this field to control printing.
    pub export_settings: Option<String>,
}

fn points(mm: f64) -> f32 {
    (mm * 72.0 / 25.4) as f32
}

/// Export a previously calculated layout, embedding each source image once.
///
/// Typed 8-bit and 16-bit buffers are embedded losslessly in tagged sRGB,
/// retaining transparency. Callers must normalize source colors to sRGB first.
/// Use typed 16-bit buffers to preserve precision: `DynamicImage`'s generic
/// pixel view exposes 8-bit samples. Repeated copies share
/// an image resource. No full-page bitmap or DPI-based resampling is performed.
/// Sources must have the same order, identifiers, dimensions, quantities, and
/// print widths as the layout inputs; their pixel contents may be replaced.
///
/// # Errors
/// Returns [`Error::LayoutMismatch`] for incompatible sources, or an error if
/// PDF serialization fails.
pub fn render_pdf<I>(items: &[ImageItem<'_, I>], layout: &DocumentLayout) -> Result<Vec<u8>, Error>
where
    I: GenericImageView + ?Sized,
    I::Pixel: Pixel,
    <I::Pixel as Pixel>::Subpixel: Into<u16>,
{
    render_pdf_with_metadata(items, layout, &PdfMetadata::default())
}

/// Create a PDF with saved settings and a request to print at 100%.
///
/// `/PrintScaling /None` tells PDF readers to print at 100%. Readers can ignore
/// it, and users can override it. Adding these settings leaves pages and images
/// unchanged.
///
/// # Errors
/// Returns the same errors as [`render_pdf`], or [`Error::PdfSettings`] if the
/// print settings cannot be saved in the PDF.
pub fn render_pdf_with_metadata<I>(
    items: &[ImageItem<'_, I>],
    layout: &DocumentLayout,
    metadata: &PdfMetadata,
) -> Result<Vec<u8>, Error>
where
    I: GenericImageView + ?Sized,
    I::Pixel: Pixel,
    <I::Pixel as Pixel>::Subpixel: Into<u16>,
{
    if !layout.matches(items) {
        return Err(Error::LayoutMismatch);
    }
    let bytes_per_component = std::mem::size_of::<<I::Pixel as Pixel>::Subpixel>() as u8;
    let images: Vec<_> = items
        .iter()
        .map(|item| {
            // Krilla's custom-image decoder uses u32 channel byte counts.
            let pixel_count = item.image.width().checked_mul(item.image.height());
            let color_count = pixel_count
                .and_then(|count| count.checked_mul(3 * u32::from(bytes_per_component)))
                .ok_or_else(|| {
                    Error::InvalidInput("image exceeds pdf channel size limits".into())
                })?;
            let rgb_capacity = usize::try_from(color_count)
                .map_err(|_| Error::InvalidInput("image exceeds addressable memory".into()))?;
            let mut rgb = Vec::new();
            let mut alpha = Vec::new();
            rgb.try_reserve_exact(rgb_capacity)
                .map_err(|_| Error::InvalidInput("cannot allocate image pixel buffer".into()))?;
            alpha
                .try_reserve_exact(rgb_capacity / 3)
                .map_err(|_| Error::InvalidInput("cannot allocate image alpha buffer".into()))?;
            for y in 0..item.image.height() {
                for x in 0..item.image.width() {
                    let pixel = item.image.get_pixel(x, y).to_rgba();
                    for channel in &pixel.0[..3] {
                        let value: u16 = (*channel).into();
                        if bytes_per_component == 2 {
                            rgb.extend_from_slice(&value.to_be_bytes());
                        } else {
                            rgb.push(value as u8);
                        }
                    }
                    let value: u16 = pixel.0[3].into();
                    if bytes_per_component == 2 {
                        alpha.extend_from_slice(&value.to_be_bytes());
                    } else {
                        alpha.push(value as u8);
                    }
                }
            }
            Image::from_custom(
                Bitmap {
                    size: item.image.dimensions(),
                    rgb: Arc::new(rgb),
                    bytes_per_component,
                    alpha: Arc::new(alpha),
                },
                true,
            )
            .map_err(Error::ImageEmbedding)
        })
        .collect::<Result<_, Error>>()?;
    let mut document = Document::new_with(SerializeSettings {
        no_device_cs: true,
        ..Default::default()
    });
    document.set_metadata(
        krilla::metadata::Metadata::new()
            .title(metadata.title.clone())
            .description(metadata.description.clone()),
    );
    for page_layout in layout.pages() {
        let settings = PageSettings::from_wh(
            points(page_layout.size.width_mm),
            points(page_layout.size.height_mm),
        )
        .ok_or_else(|| {
            Error::InvalidInput("page size cannot be represented in pdf points".into())
        })?;
        let mut page = document.start_page_with(settings);
        let mut surface = page.surface();
        for placement in &page_layout.placements {
            let x = points(placement.x_mm);
            let y = points(placement.y_mm);
            let w = points(placement.width_mm);
            let h = points(placement.height_mm);
            let (transform, width, height) = if placement.rotated {
                (Transform::from_row(0.0, 1.0, -1.0, 0.0, x + w, y), h, w)
            } else {
                (Transform::from_translate(x, y), w, h)
            };
            let size = Size::from_wh(width, height).ok_or_else(|| {
                Error::InvalidInput("image size cannot be represented in pdf points".into())
            })?;
            surface.push_transform(&transform);
            surface.draw_image(images[placement.source_index].clone(), size);
            surface.pop();
        }
        surface.finish();
        page.finish();
    }
    add_print_preferences(document.finish()?, metadata.export_settings.as_deref())
}

// Krilla 0.8 cannot set print scaling directly. Add the settings at the end of
// the PDF, leaving the existing pages and images unchanged.
fn add_print_preferences(pdf: Vec<u8>, export_settings: Option<&str>) -> Result<Vec<u8>, Error> {
    let failure = |error: lopdf::Error| Error::PdfSettings(error.to_string());
    let previous = lopdf::Document::load_mem(&pdf).map_err(failure)?;
    let root = previous
        .trailer
        .get(b"Root")
        .and_then(lopdf::Object::as_reference)
        .map_err(failure)?;
    let mut catalog = previous.get_dictionary(root).map_err(failure)?.clone();
    let mut preferences = match catalog.get(b"ViewerPreferences") {
        Ok(value) => previous
            .dereference(value)
            .and_then(|(_, value)| value.as_dict())
            .map_err(failure)?
            .clone(),
        Err(_) => lopdf::Dictionary::new(),
    };
    preferences.set("PrintScaling", lopdf::Object::Name(b"None".to_vec()));
    catalog.set("ViewerPreferences", preferences);
    let info = previous
        .trailer
        .get(b"Info")
        .ok()
        .and_then(|value| value.as_reference().ok());
    let mut update = lopdf::IncrementalDocument::create_from(pdf, previous);
    update.new_document.set_object(root, catalog);
    if let Some(snapshot) = export_settings {
        let mut dictionary = if let Some(id) = info {
            update
                .get_prev_documents()
                .get_dictionary(id)
                .map_err(failure)?
                .clone()
        } else {
            lopdf::Dictionary::new()
        };
        dictionary.set("ExportSettings", lopdf::text_string(snapshot));
        if let Some(id) = info {
            update.new_document.set_object(id, dictionary);
        } else {
            let id = update.new_document.add_object(dictionary);
            update.new_document.trailer.set("Info", id);
        }
    }
    let mut output = Vec::with_capacity(
        update.get_prev_documents_bytes().len() + export_settings.map_or(0, str::len) + 4096,
    );
    update
        .save_to(&mut output)
        .map_err(|error| Error::PdfSettings(error.to_string()))?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_preference_update_keeps_the_complete_original_pdf_unchanged() {
        let mut document = Document::new();
        document.start_page().finish();
        let original = document.finish().unwrap();
        let updated = add_print_preferences(original.clone(), Some("{\"dpi\":300}")).unwrap();
        assert!(
            updated.starts_with(&original),
            "the original image/page streams must remain byte-for-byte intact"
        );
        let parsed = lopdf::Document::load_mem(&updated).unwrap();
        assert_eq!(parsed.get_pages().len(), 1);
        assert!(
            updated[original.len()..]
                .windows(5)
                .any(|value| value == b"/Prev"),
            "the appended revision must reference the original cross-reference table"
        );
    }
}
