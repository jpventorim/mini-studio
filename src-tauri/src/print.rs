use std::{
    borrow::Cow,
    collections::HashMap,
    io::{Cursor, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
};

use base64::{Engine, engine::general_purpose::STANDARD};
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{DynamicImage, ImageDecoder, ImageEncoder, ImageFormat, ImageReader, Limits};
use image_manipulation::{ManipulationOptions, Position, PreparedImage, TrimMode, TrimOptions};
use image_packing::{DocumentLayout, ImageItem, LayoutOptions, PageSize};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State, async_runtime::spawn_blocking};
use tauri_plugin_dialog::DialogExt;

#[derive(Clone, Default)]
pub(crate) struct PrintState(Arc<Mutex<Workspace>>);

#[derive(Default)]
struct Workspace {
    next_id: u32,
    images: HashMap<u32, Arc<DynamicImage>>,
    prepared: HashMap<u32, Arc<Mutex<Option<CachedPreparation>>>>,
    preview: Option<(PrintRequest, Arc<PreparedDocument>)>,
}

struct PreparedSource {
    image_id: u32,
    copy_index: usize,
    quantity: usize,
    image: Arc<PreparedImage>,
    preview: Arc<str>,
}

#[derive(Clone)]
struct CachedCopy {
    image: Arc<PreparedImage>,
    preview: Arc<str>,
}

struct CachedPreparation {
    options: ManipulationOptions,
    template: Arc<PreparedImage>,
    plain: Option<CachedCopy>,
    numbered: Vec<CachedCopy>,
}

impl CachedPreparation {
    fn matches(&self, options: &ManipulationOptions) -> bool {
        let mut previous = self.options.clone();
        previous.max_height_mm = options.max_height_mm;
        if previous != *options {
            return false;
        }
        if self.options.max_height_mm == options.max_height_mm {
            return true;
        }
        // A paper/margin edit needs no image work when the panel was not
        // shortened and the complete strip still fits the new height limit.
        let panel = self.template.base_bounds.y - self.template.fold_y;
        let preferred = (f64::from(self.template.base_bounds.width) * 1.5).floor() as u32;
        panel == preferred && self.template.height_mm <= options.max_height_mm
    }
}

struct PreparedDocument {
    sources: Vec<PreparedSource>,
    layout: DocumentLayout,
    pdf: Mutex<Option<Arc<Vec<u8>>>>,
}

impl PrintState {
    fn workspace(&self) -> Result<MutexGuard<'_, Workspace>, PrintError> {
        self.0
            .lock()
            .map_err(|_| PrintError::new("The workspace could not be accessed."))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PrintError {
    message: String,
    image_id: Option<u32>,
}

impl PrintError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            image_id: None,
        }
    }
}

impl From<image_packing::Error> for PrintError {
    fn from(error: image_packing::Error) -> Self {
        match error {
            image_packing::Error::ImageTooLarge {
                id,
                width_mm,
                height_mm,
            } => Self {
                message: format!(
                    "This image prints at {width_mm:.1} × {height_mm:.1} mm and does not fit. Reduce its size or change the paper settings."
                ),
                image_id: id.parse().ok(),
            },
            error => Self::new(error.to_string()),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct ImportedImage {
    id: u32,
    name: String,
    width: u32,
    height: u32,
    preview: String,
}

#[derive(Default, Serialize)]
pub(crate) struct Imports {
    images: Vec<ImportedImage>,
    errors: Vec<String>,
}

fn decode(path: &Path) -> Result<(DynamicImage, String), String> {
    let mut reader = ImageReader::open(path)
        .and_then(ImageReader::with_guessed_format)
        .map_err(|error| error.to_string())?;
    if !matches!(reader.format(), Some(ImageFormat::Png | ImageFormat::Jpeg)) {
        return Err("Choose a PNG or JPEG image.".into());
    }
    let mut limits = Limits::default();
    limits.max_image_width = Some(16_384);
    limits.max_image_height = Some(16_384);
    limits.max_alloc = Some(256 * 1024 * 1024);
    reader.limits(limits);
    let mut decoder = reader.into_decoder().map_err(|error| error.to_string())?;
    let icc = decoder.icc_profile().map_err(|error| error.to_string())?;
    let orientation = decoder.orientation().map_err(|error| error.to_string())?;
    let mut image = DynamicImage::from_decoder(decoder).map_err(|error| error.to_string())?;
    image.apply_orientation(orientation);
    let image = image_manipulation::normalize_color(image, icc.as_deref())
        .map_err(|error| error.to_string())?;

    let mut preview = Cursor::new(Vec::new());
    image
        .thumbnail(960, 960)
        .write_to(&mut preview, ImageFormat::Png)
        .map_err(|error| error.to_string())?;
    let preview = STANDARD.encode(preview.into_inner());
    Ok((image, format!("data:image/png;base64,{preview}")))
}

fn load_images(state: &PrintState, paths: Vec<PathBuf>) -> Result<Imports, PrintError> {
    let mut result = Imports::default();
    for path in paths {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Image".into());
        match decode(&path) {
            Ok((image, preview)) => {
                let mut workspace = state.workspace()?;
                let id = workspace.next_id;
                workspace.next_id = id
                    .checked_add(1)
                    .ok_or_else(|| PrintError::new("Too many images have been added."))?;
                result.images.push(ImportedImage {
                    id,
                    name,
                    width: image.width(),
                    height: image.height(),
                    preview,
                });
                workspace.images.insert(id, Arc::new(image));
            }
            Err(error) => result.errors.push(format!("{name} — {error}")),
        }
    }
    Ok(result)
}

#[tauri::command]
pub(crate) async fn choose_images(
    app: AppHandle,
    state: State<'_, PrintState>,
) -> Result<Imports, PrintError> {
    let state = state.inner().clone();
    blocking(move || {
        let paths = app
            .dialog()
            .file()
            .set_title("Add images")
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .blocking_pick_files()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|path| path.into_path().ok())
            .collect();
        load_images(&state, paths)
    })
    .await
}

#[tauri::command]
pub(crate) async fn import_images(
    state: State<'_, PrintState>,
    paths: Vec<PathBuf>,
) -> Result<Imports, PrintError> {
    let state = state.inner().clone();
    blocking(move || load_images(&state, paths)).await
}

#[tauri::command]
pub(crate) fn remove_image(state: State<'_, PrintState>, id: u32) -> Result<(), PrintError> {
    let mut workspace = state.workspace()?;
    workspace.images.remove(&id);
    workspace.prepared.remove(&id);
    workspace.preview = None;
    Ok(())
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PrintRequest {
    items: Vec<PrintItem>,
    width_mm: f64,
    height_mm: f64,
    dpi: f64,
    margin_mm: f64,
    spacing_mm: f64,
    grid_size_mm: f64,
}

impl PrintRequest {
    fn options(&self) -> LayoutOptions {
        LayoutOptions {
            page_size: PageSize {
                width_mm: self.width_mm,
                height_mm: self.height_mm,
            },
            dpi: self.dpi,
            margin_mm: self.margin_mm,
            spacing_mm: self.spacing_mm,
            allow_rotation: true,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct PrintItem {
    id: u32,
    quantity: usize,
    size: f64,
    number_copies: bool,
    border_px: u32,
    artwork_width_ratio: f64,
    trim: Background,
    background_color: [u8; 3],
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Background {
    Auto,
    Alpha,
    Color,
    None,
}

fn sources(
    state: &PrintState,
    request: &PrintRequest,
) -> Result<Vec<Arc<DynamicImage>>, PrintError> {
    let count = request
        .items
        .iter()
        .try_fold(0usize, |total, item| total.checked_add(item.quantity));
    if count.is_none_or(|count| count > 5_000) {
        return Err(PrintError::new("Use 5,000 copies or fewer per PDF."));
    }
    let workspace = state.workspace()?;
    request
        .items
        .iter()
        .map(|item| {
            workspace
                .images
                .get(&item.id)
                .cloned()
                .ok_or_else(|| PrintError {
                    message: "This image is no longer available.".into(),
                    image_id: Some(item.id),
                })
        })
        .collect()
}

fn encode_preview(image: &image::RgbaImage) -> Result<Arc<str>, PrintError> {
    let thumbnail = if image.width() <= 480 && image.height() <= 960 {
        Cow::Borrowed(image)
    } else {
        let scale = (480.0 / f64::from(image.width())).min(960.0 / f64::from(image.height()));
        let width = (f64::from(image.width()) * scale).round().max(1.0) as u32;
        let height = (f64::from(image.height()) * scale).round().max(1.0) as u32;
        Cow::Owned(image::imageops::thumbnail(image, width, height))
    };
    let mut bytes = Vec::new();
    // The fixed filter avoids trying every PNG filter for every scanline.
    // PNG remains lossless; only compressed size changes.
    PngEncoder::new_with_quality(&mut bytes, CompressionType::Fast, FilterType::Sub)
        .write_image(
            thumbnail.as_raw(),
            thumbnail.width(),
            thumbnail.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|error| PrintError::new(error.to_string()))?;
    Ok(format!("data:image/png;base64,{}", STANDARD.encode(bytes)).into())
}

fn prepare_sources(
    state: &PrintState,
    request: &PrintRequest,
    sources: &[Arc<DynamicImage>],
) -> Result<Vec<PreparedSource>, PrintError> {
    let mut prepared = Vec::new();
    let mut total_pixels = 0_u64;
    for (item, source) in request.items.iter().zip(sources) {
        if ![0.5, 0.75, 1.0, 2.0, 3.0, 4.0, 5.0].contains(&item.size) {
            return Err(PrintError {
                message: "Choose size 0.5, 0.75, 1, 2, 3, 4, or 5.".into(),
                image_id: Some(item.id),
            });
        }
        let options = ManipulationOptions {
            size: item.size,
            grid_size_mm: request.grid_size_mm,
            dpi: request.dpi,
            position: Position::Grounded,
            number_copies: false,
            border_px: item.border_px,
            artwork_width_ratio: item.artwork_width_ratio,
            // Rotation can use either printable dimension as the strip height.
            max_height_mm: request.width_mm.max(request.height_mm) - 2.0 * request.margin_mm,
            trim: TrimOptions {
                mode: match item.trim {
                    Background::Auto => TrimMode::Auto,
                    Background::Alpha => TrimMode::Alpha,
                    Background::Color => TrimMode::Color(item.background_color),
                    Background::None => TrimMode::None,
                },
                ..TrimOptions::default()
            },
            ..ManipulationOptions::default()
        };
        let source_error = |error: image_manipulation::Error| PrintError {
            message: error.to_string(),
            image_id: Some(item.id),
        };
        if item.quantity == 0 {
            return Err(PrintError {
                message: "Use at least one copy.".into(),
                image_id: Some(item.id),
            });
        }
        let count = if item.number_copies { item.quantity } else { 1 };
        let shared = {
            let mut workspace = state.workspace()?;
            if !workspace
                .images
                .get(&item.id)
                .is_some_and(|current| Arc::ptr_eq(current, source))
            {
                return Err(PrintError {
                    message: "This image is no longer available.".into(),
                    image_id: Some(item.id),
                });
            }
            Arc::clone(workspace.prepared.entry(item.id).or_default())
        };
        let mut slot = shared
            .lock()
            .map_err(|_| PrintError::new("Prepared images could not be accessed."))?;
        if slot.as_ref().is_none_or(|cache| !cache.matches(&options)) {
            let template =
                image_manipulation::prepare(source.as_ref(), &options).map_err(source_error)?;
            *slot = Some(CachedPreparation {
                options,
                template: Arc::new(template),
                plain: None,
                numbered: Vec::new(),
            });
        }
        let cache = slot
            .as_mut()
            .ok_or_else(|| PrintError::new("Image preparation did not finish."))?;
        let pixels =
            u64::from(cache.template.image.width()) * u64::from(cache.template.image.height());
        // Include the blank template for numbered sources. Pixel buffers are
        // shared with layouts; keep only the copies requested by this edit.
        let buffers = if item.number_copies {
            count as u64 + 1
        } else {
            1
        };
        total_pixels = total_pixels
            .checked_add(
                pixels
                    .checked_mul(buffers)
                    .ok_or_else(|| PrintError::new("Too many image pixels."))?,
            )
            .ok_or_else(|| PrintError::new("Too many image pixels."))?;
        if total_pixels > 64 * 1024 * 1024 {
            return Err(PrintError {
                message: "These standees need too much memory. Reduce copies or DPI.".into(),
                image_id: Some(item.id),
            });
        }
        if item.number_copies {
            cache.numbered.truncate(count);
            while cache.numbered.len() < count {
                let image = cache
                    .template
                    .numbered_copy(cache.numbered.len() + 1)
                    .map_err(source_error)?;
                let preview = encode_preview(&image.image)?;
                cache.numbered.push(CachedCopy {
                    image: Arc::new(image),
                    preview,
                });
            }
            for (copy_index, copy) in cache.numbered.iter().enumerate() {
                prepared.push(PreparedSource {
                    image_id: item.id,
                    copy_index,
                    quantity: 1,
                    image: Arc::clone(&copy.image),
                    preview: Arc::clone(&copy.preview),
                });
            }
        } else {
            cache.numbered.clear();
            if cache.plain.is_none() {
                cache.plain = Some(CachedCopy {
                    image: Arc::clone(&cache.template),
                    preview: encode_preview(&cache.template.image)?,
                });
            }
            if let Some(copy) = &cache.plain {
                prepared.push(PreparedSource {
                    image_id: item.id,
                    copy_index: 0,
                    quantity: item.quantity,
                    image: Arc::clone(&copy.image),
                    preview: Arc::clone(&copy.preview),
                });
            }
        }
    }
    Ok(prepared)
}

fn items(sources: &[PreparedSource]) -> Vec<ImageItem<'_, image::RgbaImage>> {
    sources
        .iter()
        .map(|source| {
            ImageItem::new(
                source.image_id.to_string(),
                &source.image.image,
                source.quantity,
            )
            .with_width_mm(source.image.width_mm)
            .with_rotation(true)
        })
        .collect()
}

fn prepared_document(
    state: &PrintState,
    request: &PrintRequest,
) -> Result<Arc<PreparedDocument>, PrintError> {
    if let Some((_, document)) = state
        .workspace()?
        .preview
        .as_ref()
        .filter(|(cached, _)| cached == request)
    {
        return Ok(Arc::clone(document));
    }
    // Prepare first, then pass only pixels and print dimensions to packing.
    let originals = sources(state, request)?;
    let sources = match prepare_sources(state, request, &originals) {
        Ok(sources) => sources,
        Err(error) => {
            // Do not retain partially grown caches after a failed request.
            state.workspace()?.prepared.clear();
            return Err(error);
        }
    };
    let layout = image_packing::layout(&items(&sources), &request.options())?;
    let document = Arc::new(PreparedDocument {
        sources,
        layout,
        pdf: Mutex::default(),
    });
    // Share one document (including its lazy PDF) between preview and export.
    // Recheck source identity after preparation in case an image was removed.
    let mut workspace = state.workspace()?;
    if let Some((item, _)) = request
        .items
        .iter()
        .zip(&originals)
        .find(|(item, original)| {
            !workspace
                .images
                .get(&item.id)
                .is_some_and(|current| Arc::ptr_eq(current, original))
        })
    {
        return Err(PrintError {
            message: "This image is no longer available.".into(),
            image_id: Some(item.id),
        });
    }
    // Another worker may have completed the same request while we prepared.
    if let Some((_, existing)) = workspace
        .preview
        .as_ref()
        .filter(|(cached, _)| cached == request)
    {
        return Ok(Arc::clone(existing));
    }
    workspace.preview = Some((request.clone(), Arc::clone(&document)));
    Ok(document)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PreparedPreview {
    pages: Vec<PreviewPage>,
    images: Vec<PreviewImage>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PreviewImage {
    image_id: u32,
    preview: String,
    width_mm: f64,
    height_mm: f64,
    copy_number: Option<usize>,
    base_width_mm: f64,
    base_depth_mm: f64,
    folded_height_mm: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PreviewPage {
    width_mm: f64,
    height_mm: f64,
    placements: Vec<PreviewPlacement>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PreviewPlacement {
    source_index: usize,
    image_id: u32,
    copy_index: usize,
    x_mm: f64,
    y_mm: f64,
    width_mm: f64,
    height_mm: f64,
    rotated: bool,
}

#[tauri::command]
pub(crate) async fn preview_layout(
    state: State<'_, PrintState>,
    request: PrintRequest,
) -> Result<PreparedPreview, PrintError> {
    let state = state.inner().clone();
    blocking(move || {
        let document = prepared_document(&state, &request)?;
        Ok(document_preview(&document))
    })
    .await
}

fn document_preview(document: &PreparedDocument) -> PreparedPreview {
    let pages = document
        .layout
        .pages()
        .iter()
        .map(|page| PreviewPage {
            width_mm: page.size.width_mm,
            height_mm: page.size.height_mm,
            placements: page
                .placements
                .iter()
                .map(|placement| PreviewPlacement {
                    source_index: placement.source_index,
                    image_id: document.sources[placement.source_index].image_id,
                    copy_index: document.sources[placement.source_index].copy_index
                        + placement.copy_index,
                    x_mm: placement.x_mm,
                    y_mm: placement.y_mm,
                    width_mm: placement.width_mm,
                    height_mm: placement.height_mm,
                    rotated: placement.rotated,
                })
                .collect(),
        })
        .collect();
    let images = document
        .sources
        .iter()
        .map(|source| PreviewImage {
            image_id: source.image_id,
            preview: source.preview.to_string(),
            width_mm: source.image.width_mm,
            height_mm: source.image.height_mm,
            copy_number: source.image.copy_number,
            base_width_mm: f64::from(source.image.base_bounds.width) * 25.4 / source.image.dpi,
            base_depth_mm: f64::from(source.image.base_bounds.height) * 25.4 / source.image.dpi,
            folded_height_mm: f64::from(source.image.base_bounds.y - source.image.fold_y) * 25.4
                / source.image.dpi,
        })
        .collect();
    PreparedPreview { pages, images }
}

#[tauri::command]
pub(crate) async fn export_pdf(
    app: AppHandle,
    state: State<'_, PrintState>,
    request: PrintRequest,
) -> Result<Option<String>, PrintError> {
    let state = state.inner().clone();
    blocking(move || {
        let document = prepared_document(&state, &request)?;
        let Some(path) = app
            .dialog()
            .file()
            .set_title("Export PDF")
            .add_filter("PDF document", &["pdf"])
            .set_file_name("print-sheet.pdf")
            .blocking_save_file()
        else {
            return Ok(None);
        };
        let path = path
            .into_path()
            .map_err(|error| PrintError::new(error.to_string()))?;
        let pdf = pdf_bytes(&state, &request, &document)?;
        save_pdf(&path, pdf.as_slice())?;
        Ok(Some(path.to_string_lossy().into_owned()))
    })
    .await
}

fn pdf_bytes(
    state: &PrintState,
    request: &PrintRequest,
    document: &PreparedDocument,
) -> Result<Arc<Vec<u8>>, PrintError> {
    // Serialize generation for this document only. Other settings/documents can
    // still prepare independently; failures leave the cache empty for a retry.
    let mut cache = document
        .pdf
        .lock()
        .map_err(|_| PrintError::new("The PDF cache could not be accessed."))?;
    if let Some(pdf) = cache.as_ref() {
        return Ok(Arc::clone(pdf));
    }
    let print_images = render_print_images(state, request, document)?;
    let print_items: Vec<_> = document
        .sources
        .iter()
        .zip(&print_images)
        .map(|(source, image)| {
            ImageItem::new(source.image_id.to_string(), image, source.quantity)
                .with_width_mm(source.image.width_mm)
                .with_rotation(true)
        })
        .collect();
    let metadata = export_metadata(request, document)?;
    let pdf = Arc::new(image_packing::render_pdf_with_metadata(
        &print_items,
        &document.layout,
        &metadata,
    )?);
    *cache = Some(Arc::clone(&pdf));
    Ok(pdf)
}

fn export_metadata(
    request: &PrintRequest,
    document: &PreparedDocument,
) -> Result<image_packing::PdfMetadata, PrintError> {
    let defaults = ManipulationOptions::default();
    let prepared: Vec<_> = document
        .sources
        .iter()
        .map(|source| {
            let image = &source.image;
            serde_json::json!({
                "imageId": source.image_id,
                "copyNumber": image.copy_number,
                "quantity": source.quantity,
                "pixelWidth": image.image.width(),
                "pixelHeight": image.image.height(),
                "widthMm": image.width_mm,
                "heightMm": image.height_mm,
                "effectiveRasterPpi": image.dpi,
                "baseWidthMm": f64::from(image.base_bounds.width) * 25.4 / image.dpi,
                "baseTabDepthMm": f64::from(image.base_bounds.height) * 25.4 / image.dpi,
                "foldedHeightMm": f64::from(image.base_bounds.y - image.fold_y) * 25.4 / image.dpi,
            })
        })
        .collect();
    let snapshot = serde_json::json!({
        "schemaVersion": 1,
        "application": { "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION") },
        "targetRasterPpi": request.dpi,
        "printScaling": "none",
        "colorSpace": "sRGB",
        "imageBitsPerChannel": 16,
        "rotationAllowed": true,
        "processing": {
            "position": "grounded",
            "baseShape": "rectangle",
            "baseClearance": "automatic",
            "alphaThreshold": defaults.trim.alpha_threshold,
            "colorTolerance": defaults.trim.color_tolerance,
            "maxStripHeightMm": request.width_mm.max(request.height_mm) - 2.0 * request.margin_mm,
        },
        // Save all selected settings, including those for each image.
        "settings": request,
        "pageCount": document.layout.pages().len(),
        "preparedImages": prepared,
    });
    Ok(image_packing::PdfMetadata {
        title: "Paper miniatures print sheet".into(),
        description: format!(
            "Print at actual size (100%). Paper: {} x {} mm; image resolution: {} PPI; margin: {} mm; spacing: {} mm; grid: {} mm; color: sRGB; image channels: 16-bit.",
            request.width_mm,
            request.height_mm,
            request.dpi,
            request.margin_mm,
            request.spacing_mm,
            request.grid_size_mm,
        ),
        export_settings: Some(serde_json::to_string(&snapshot).map_err(|error| {
            PrintError::new(format!("Could not record export settings: {error}"))
        })?),
    })
}

fn render_print_images(
    state: &PrintState,
    request: &PrintRequest,
    document: &PreparedDocument,
) -> Result<Vec<image_manipulation::PrintImage>, PrintError> {
    let originals = sources(state, request)?;
    let originals: HashMap<_, _> = request
        .items
        .iter()
        .zip(originals)
        .map(|(item, source)| (item.id, source))
        .collect();
    let mut artwork = HashMap::new();
    let mut print_images = Vec::with_capacity(document.sources.len());
    for prepared in &document.sources {
        if let std::collections::hash_map::Entry::Vacant(entry) = artwork.entry(prepared.image_id) {
            let original = originals
                .get(&prepared.image_id)
                .ok_or_else(|| PrintError::new("This image is no longer available."))?;
            entry.insert(
                prepared
                    .image
                    .prepare_print_artwork(original)
                    .map_err(|error| PrintError::new(error.to_string()))?,
            );
        }
        print_images.push(
            prepared
                .image
                .render_print_image(&artwork[&prepared.image_id])
                .map_err(|error| PrintError::new(error.to_string()))?,
        );
    }
    Ok(print_images)
}

fn save_pdf(path: &Path, pdf: &[u8]) -> Result<(), PrintError> {
    let folder = path
        .parent()
        .ok_or_else(|| PrintError::new("Choose a folder for the PDF."))?;
    let save_error =
        |error: std::io::Error| PrintError::new(format!("Could not save the PDF. {error}"));
    let mut file = tempfile::NamedTempFile::new_in(folder).map_err(save_error)?;
    file.write_all(pdf).map_err(save_error)?;
    file.persist(path)
        .map_err(|error| save_error(error.error))?;
    Ok(())
}

async fn blocking<T: Send + 'static>(
    job: impl FnOnce() -> Result<T, PrintError> + Send + 'static,
) -> Result<T, PrintError> {
    spawn_blocking(job)
        .await
        .map_err(|_| PrintError::new("The print task could not finish."))?
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: u32, quantity: usize, numbered: bool) -> PrintItem {
        PrintItem {
            id,
            quantity,
            size: 1.0,
            number_copies: numbered,
            border_px: 3,
            artwork_width_ratio: 0.9,
            trim: Background::Auto,
            background_color: [255; 3],
        }
    }

    fn make_request(items: Vec<PrintItem>) -> PrintRequest {
        PrintRequest {
            items,
            width_mm: 210.0,
            height_mm: 297.0,
            dpi: 100.0,
            margin_mm: 2.0,
            spacing_mm: 0.0,
            grid_size_mm: 25.0,
        }
    }

    fn source() -> DynamicImage {
        DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            20,
            40,
            image::Rgba([80, 20, 150, 255]),
        ))
    }

    fn prepare_test_sources(
        request: &PrintRequest,
        sources: &[Arc<DynamicImage>],
    ) -> Result<Vec<PreparedSource>, PrintError> {
        let state = PrintState::default();
        for (item, source) in request.items.iter().zip(sources) {
            state
                .workspace()?
                .images
                .insert(item.id, Arc::clone(source));
        }
        prepare_sources(&state, request, sources)
    }

    #[test]
    fn export_settings_snapshot_records_every_requested_control_and_effective_resolution() {
        let state = PrintState::default();
        for id in [10, 20] {
            state
                .workspace()
                .unwrap()
                .images
                .insert(id, Arc::new(source()));
        }
        let mut request = make_request(vec![item(10, 2, true), item(20, 3, false)]);
        request.width_mm = 297.0;
        request.height_mm = 420.0;
        request.dpi = 300.0;
        request.margin_mm = 0.5;
        request.spacing_mm = 0.5;
        request.grid_size_mm = 20.0;
        request.items[0].size = 0.75;
        request.items[0].border_px = 4;
        request.items[0].artwork_width_ratio = 0.85;
        request.items[1].trim = Background::Color;
        request.items[1].background_color = [250, 251, 252];
        let document = prepared_document(&state, &request).unwrap();
        let metadata = export_metadata(&request, &document).unwrap();
        assert!(metadata.description.contains("300 PPI"));
        assert!(metadata.description.contains("297 x 420 mm"));
        let snapshot: serde_json::Value =
            serde_json::from_str(metadata.export_settings.as_deref().unwrap()).unwrap();
        assert_eq!(snapshot["schemaVersion"], 1);
        assert_eq!(snapshot["targetRasterPpi"].as_f64(), Some(300.0));
        assert_eq!(snapshot["printScaling"], "none");
        assert_eq!(
            snapshot["settings"],
            serde_json::to_value(&request).unwrap(),
            "no requested field may be omitted"
        );
        assert_eq!(snapshot["imageBitsPerChannel"], 16);
        assert_eq!(snapshot["colorSpace"], "sRGB");
        assert_eq!(snapshot["processing"]["alphaThreshold"], 0);
        assert_eq!(snapshot["processing"]["colorTolerance"], 8);
        let bytes = pdf_bytes(&state, &request, &document).unwrap();
        let pdf = lopdf::Document::load_mem(bytes.as_slice()).unwrap();
        let info = pdf
            .get_dictionary(pdf.trailer.get(b"Info").unwrap().as_reference().unwrap())
            .unwrap();
        let recorded = lopdf::decode_text_string(info.get(b"ExportSettings").unwrap()).unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&recorded).unwrap(),
            snapshot
        );
        assert!(
            lopdf::decode_text_string(info.get(b"Subject").unwrap())
                .unwrap()
                .contains("300 PPI")
        );
        let preferences = pdf
            .catalog()
            .unwrap()
            .get(b"ViewerPreferences")
            .unwrap()
            .as_dict()
            .unwrap();
        assert_eq!(
            preferences.get(b"PrintScaling").unwrap().as_name().unwrap(),
            b"None"
        );
        let prepared = snapshot["preparedImages"].as_array().unwrap();
        assert_eq!(
            prepared.len(),
            3,
            "two numbered copies and one repeated plain source"
        );
        assert_eq!(prepared[0]["copyNumber"], 1);
        assert_eq!(prepared[1]["copyNumber"], 2);
        assert_eq!(prepared[2]["quantity"], 3);
        for (saved, source) in prepared.iter().zip(&document.sources) {
            assert_eq!(saved["pixelWidth"], source.image.image.width());
            assert_eq!(saved["effectiveRasterPpi"], source.image.dpi);
        }
    }

    #[test]
    fn unchanged_exports_reuse_pdf_bytes_and_can_save_to_different_paths() {
        let state = PrintState::default();
        state
            .workspace()
            .unwrap()
            .images
            .insert(10, Arc::new(source()));
        let request = make_request(vec![item(10, 2, true)]);
        let document = prepared_document(&state, &request).unwrap();
        assert!(
            document.pdf.lock().unwrap().is_none(),
            "preview preparation must not render PDF"
        );
        let first = pdf_bytes(&state, &request, &document).unwrap();
        let repeated = prepared_document(&state, &request).unwrap();
        assert!(
            Arc::ptr_eq(&document, &repeated),
            "export reuse must not require a separate preview call"
        );
        let second = pdf_bytes(&state, &request, &repeated).unwrap();
        assert!(
            Arc::ptr_eq(&first, &second),
            "reuse the allocation, not just equivalent PDF content"
        );
        let folder = tempfile::tempdir().unwrap();
        for name in ["first.pdf", "second.pdf"] {
            let path = folder.path().join(name);
            save_pdf(&path, first.as_slice()).unwrap();
            assert_eq!(std::fs::read(path).unwrap(), *first);
        }
    }

    #[test]
    fn setting_and_image_edits_replace_the_pdf_cache_and_release_old_bytes() {
        let state = PrintState::default();
        for id in [10, 20] {
            state
                .workspace()
                .unwrap()
                .images
                .insert(id, Arc::new(source()));
        }
        let request = make_request(vec![item(10, 1, false)]);
        let document = prepared_document(&state, &request).unwrap();
        let pdf = pdf_bytes(&state, &request, &document).unwrap();
        let old_bytes = Arc::downgrade(&pdf);
        let edits: [fn(&mut PrintRequest); 10] = [
            |r| r.items[0].quantity += 1,
            |r| r.items[0].number_copies = true,
            |r| r.items[0].size = 2.0,
            |r| r.items[0].border_px += 1,
            |r| r.items[0].artwork_width_ratio = 0.8,
            |r| r.spacing_mm = 0.5,
            |r| r.margin_mm = 0.5,
            |r| r.height_mm = 420.0,
            |r| r.dpi = 150.0,
            |r| r.grid_size_mm = 20.0,
        ];
        for edit in edits {
            let mut changed = request.clone();
            edit(&mut changed);
            let updated = prepared_document(&state, &changed).unwrap();
            assert!(!Arc::ptr_eq(&document, &updated));
            assert!(
                updated.pdf.lock().unwrap().is_none(),
                "changed settings must not reuse PDF bytes"
            );
        }
        let mut changed = request.clone();
        changed.items.push(item(20, 1, false));
        let updated = prepared_document(&state, &changed).unwrap();
        assert!(!Arc::ptr_eq(&document, &updated));
        assert!(updated.pdf.lock().unwrap().is_none());
        drop(pdf);
        drop(document);
        assert!(
            old_bytes.upgrade().is_none(),
            "old PDF bytes must be released with their document"
        );
    }

    #[test]
    fn concurrent_exports_share_one_pdf_allocation() {
        let state = PrintState::default();
        state
            .workspace()
            .unwrap()
            .images
            .insert(10, Arc::new(source()));
        let request = make_request(vec![item(10, 2, true)]);
        let barrier = std::sync::Barrier::new(4);
        let results = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    scope.spawn(|| {
                        barrier.wait();
                        let document = prepared_document(&state, &request).unwrap();
                        pdf_bytes(&state, &request, &document).unwrap()
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .collect::<Vec<_>>()
        });
        assert!(results.iter().all(|pdf| Arc::ptr_eq(&results[0], pdf)));
    }

    #[test]
    fn failed_generation_is_retryable_and_failed_saving_preserves_cached_bytes() {
        let state = PrintState::default();
        state
            .workspace()
            .unwrap()
            .images
            .insert(10, Arc::new(source()));
        let request = make_request(vec![item(10, 1, false)]);
        let document = prepared_document(&state, &request).unwrap();
        let original = state.workspace().unwrap().images.remove(&10).unwrap();
        assert!(pdf_bytes(&state, &request, &document).is_err());
        assert!(document.pdf.lock().unwrap().is_none());
        state.workspace().unwrap().images.insert(10, original);
        let pdf = pdf_bytes(&state, &request, &document).unwrap();
        let folder = tempfile::tempdir().unwrap();
        assert!(save_pdf(&folder.path().join("missing/sheet.pdf"), pdf.as_slice()).is_err());
        let retry = pdf_bytes(&state, &request, &document).unwrap();
        assert!(Arc::ptr_eq(&pdf, &retry));
        let destination = folder.path().join("sheet.pdf");
        save_pdf(&destination, retry.as_slice()).unwrap();
        assert_eq!(std::fs::read(destination).unwrap(), *pdf);
    }

    #[test]
    fn export_uses_original_sixteen_bit_source_while_preserving_preview_geometry_and_numbers() {
        let state = PrintState::default();
        let original =
            image::DynamicImage::ImageRgba16(image_manipulation::PrintImage::from_pixel(
                10,
                20,
                image::Rgba([12345, 23456, 34567, 45678]),
            ));
        state
            .workspace()
            .unwrap()
            .images
            .insert(10, Arc::new(original));
        let request = make_request(vec![item(10, 2, true)]);
        let document = prepared_document(&state, &request).unwrap();
        let print = render_print_images(&state, &request, &document).unwrap();
        assert_eq!(print.len(), 2);
        for (image, prepared) in print.iter().zip(&document.sources) {
            assert_eq!(image.dimensions(), prepared.image.image.dimensions());
            let b = prepared.image.artwork_bounds;
            let pixel = image.get_pixel(b.x + b.width / 2, b.y + b.height / 2);
            assert!(
                pixel[0].abs_diff(12345) <= 2,
                "must retain original precision: {pixel:?}"
            );
        }
        assert_ne!(print[0], print[1], "numbered bases must remain distinct");
        let items: Vec<_> = document
            .sources
            .iter()
            .zip(&print)
            .map(|(source, image)| {
                ImageItem::new(source.image_id.to_string(), image, source.quantity)
                    .with_width_mm(source.image.width_mm)
                    .with_rotation(true)
            })
            .collect();
        let pdf = image_packing::render_pdf(&items, &document.layout).unwrap();
        assert!(pdf.starts_with(b"%PDF-"));
    }

    #[test]
    fn imported_image_is_prepared_then_packed_and_saved_without_touching_the_source() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("token.png");
        source().save(&path).unwrap();
        let original = std::fs::read(&path).unwrap();
        let state = PrintState::default();
        let imported = load_images(&state, vec![path.clone()]).unwrap();
        assert!(imported.errors.is_empty());
        let image = &imported.images[0];
        let request = make_request(vec![item(image.id, 2, false)]);
        let document = prepared_document(&state, &request).unwrap();
        assert_eq!(document.sources.len(), 1);
        let prepared = &document.sources[0].image;
        assert_eq!(prepared.image.get_pixel(0, 0), &image::Rgba([0, 0, 0, 255]));
        assert!(prepared.height_mm > 91.0);
        let preview = document_preview(&document);
        let final_height = preview.images[0].folded_height_mm;
        assert!((final_height - 34.5).abs() < 25.4 / prepared.dpi);
        assert!(final_height < prepared.height_mm / 2.0);
        for y in 0..prepared.image.height() {
            for x in 0..prepared.image.width() {
                assert_eq!(
                    prepared.image.get_pixel(x, y),
                    prepared.image.get_pixel(x, prepared.image.height() - 1 - y)
                );
            }
        }
        let packing_items = items(&document.sources);
        assert_eq!(document.layout.pages()[0].placements.len(), 2);
        assert_eq!(packing_items[0].width_mm, Some(prepared.width_mm));
        assert!(
            document.sources[0]
                .preview
                .starts_with("data:image/png;base64,")
        );
        let destination = folder.path().join("sheet.pdf");
        let pdf = pdf_bytes(&state, &request, &document).unwrap();
        save_pdf(&destination, pdf.as_slice()).unwrap();
        assert!(std::fs::read(destination).unwrap().starts_with(b"%PDF-"));
        assert_eq!(std::fs::read(path).unwrap(), original);
        state.workspace().unwrap().preview = Some((request, Arc::clone(&document)));
        // The same prepared pixels and layout can be reused for export.
        let same_request = make_request(vec![item(image.id, 2, false)]);
        assert!(Arc::ptr_eq(
            &document,
            &prepared_document(&state, &same_request).unwrap()
        ));
    }

    #[test]
    fn numbered_sources_restart_and_every_copy_reaches_packing() {
        let request = make_request(vec![item(10, 4, true), item(20, 3, true)]);
        let sources = vec![Arc::new(source()), Arc::new(source())];
        let prepared = prepare_test_sources(&request, &sources).unwrap();
        assert_eq!(
            prepared
                .iter()
                .map(|s| (s.image_id, s.image.copy_number))
                .collect::<Vec<_>>(),
            vec![
                (10, Some(1)),
                (10, Some(2)),
                (10, Some(3)),
                (10, Some(4)),
                (20, Some(1)),
                (20, Some(2)),
                (20, Some(3))
            ]
        );
        assert!(prepared.iter().all(|s| s.quantity == 1));
        let items = items(&prepared);
        let layout = image_packing::layout(&items, &request.options()).unwrap();
        assert_eq!(
            layout
                .pages()
                .iter()
                .map(|p| p.placements.len())
                .sum::<usize>(),
            7
        );
        let pdf = image_packing::render_pdf(&items, &layout).unwrap();
        assert!(pdf.starts_with(b"%PDF-"));
        let document = PreparedDocument {
            sources: prepared,
            layout,
            pdf: Mutex::default(),
        };
        let preview = document_preview(&document);
        assert_eq!(preview.images.len(), 7);
        let mut actual_copies = Vec::new();
        for page in &preview.pages {
            for placement in &page.placements {
                let image = &preview.images[placement.source_index];
                assert_eq!(image.image_id, placement.image_id);
                assert_eq!(image.copy_number, Some(placement.copy_index + 1));
                actual_copies.push((placement.image_id, placement.copy_index));
            }
        }
        actual_copies.sort_unstable();
        assert_eq!(
            actual_copies,
            vec![
                (10, 0),
                (10, 1),
                (10, 2),
                (10, 3),
                (20, 0),
                (20, 1),
                (20, 2)
            ]
        );
    }

    #[test]
    fn small_creature_numbered_images_reach_preview_and_pdf_on_both_grids() {
        for grid_size_mm in [20.0, 25.0] {
            let state = PrintState::default();
            state
                .workspace()
                .unwrap()
                .images
                .insert(10, Arc::new(source()));
            let mut request = make_request(vec![item(10, 12, true)]);
            request.items[0].size = 0.75;
            request.grid_size_mm = grid_size_mm;
            request.dpi = 300.0;
            let document = prepared_document(&state, &request)
                .expect("small creature numbered standees should prepare and pack");
            let preview = document_preview(&document);
            assert_eq!(preview.images.len(), 12);
            assert_eq!(
                preview
                    .pages
                    .iter()
                    .map(|page| page.placements.len())
                    .sum::<usize>(),
                12
            );
            assert_eq!(
                preview
                    .images
                    .iter()
                    .map(|image| image.copy_number)
                    .collect::<Vec<_>>(),
                (1..=12).map(Some).collect::<Vec<_>>()
            );
            let pdf =
                image_packing::render_pdf(&items(&document.sources), &document.layout).unwrap();
            assert!(pdf.starts_with(b"%PDF-"));
        }
    }

    #[test]
    fn layout_edits_and_unnumbered_quantities_reuse_pixels_and_previews() {
        let state = PrintState::default();
        for id in [10, 20] {
            state
                .workspace()
                .unwrap()
                .images
                .insert(id, Arc::new(source()));
        }
        let mut request = make_request(vec![item(10, 2, false), item(20, 1, false)]);
        let first = prepared_document(&state, &request).unwrap();
        request.spacing_mm = 0.5;
        request.items[0].quantity = 3;
        let edited = prepared_document(&state, &request).unwrap();
        for i in 0..2 {
            assert!(Arc::ptr_eq(
                &first.sources[i].image,
                &edited.sources[i].image
            ));
            assert!(Arc::ptr_eq(
                &first.sources[i].preview,
                &edited.sources[i].preview
            ));
        }
        assert_eq!(edited.sources[0].quantity, 3);
        request.height_mm = 420.0;
        let taller_paper = prepared_document(&state, &request).unwrap();
        assert!(Arc::ptr_eq(
            &first.sources[0].image,
            &taller_paper.sources[0].image
        ));
        request.items[0].size = 2.0;
        let resized = prepared_document(&state, &request).unwrap();
        assert!(!Arc::ptr_eq(
            &first.sources[0].image,
            &resized.sources[0].image
        ));
        assert!(Arc::ptr_eq(
            &first.sources[1].image,
            &resized.sources[1].image
        ));
    }

    #[test]
    fn paper_height_changes_rebuild_capped_geometry_without_shrinking_the_base() {
        let state = PrintState::default();
        state
            .workspace()
            .unwrap()
            .images
            .insert(10, Arc::new(source()));
        let mut request = make_request(vec![item(10, 1, false)]);
        request.items[0].size = 3.0;
        let capped = prepared_document(&state, &request).unwrap();
        request.height_mm = 420.0;
        let taller = prepared_document(&state, &request).unwrap();
        assert!(!Arc::ptr_eq(
            &capped.sources[0].image,
            &taller.sources[0].image
        ));
        assert!(taller.sources[0].image.height_mm > capped.sources[0].image.height_mm);
        request.height_mm = 297.0;
        let capped_again = prepared_document(&state, &request).unwrap();
        assert!(!Arc::ptr_eq(
            &taller.sources[0].image,
            &capped_again.sources[0].image
        ));
        assert_eq!(
            capped_again.sources[0].image.height_mm,
            capped.sources[0].image.height_mm
        );
        assert_eq!(
            capped.sources[0].image.base_bounds.width,
            taller.sources[0].image.base_bounds.width
        );
        assert_eq!(
            capped.sources[0].image.base_bounds.height,
            taller.sources[0].image.base_bounds.height
        );
    }

    #[test]
    fn numbered_quantity_edits_reuse_existing_copies_and_release_unused_copies() {
        let state = PrintState::default();
        state
            .workspace()
            .unwrap()
            .images
            .insert(10, Arc::new(source()));
        let mut request = make_request(vec![item(10, 3, true)]);
        let first = prepared_document(&state, &request).unwrap();
        let shared = Arc::clone(state.workspace().unwrap().prepared.get(&10).unwrap());
        let template = Arc::clone(&shared.lock().unwrap().as_ref().unwrap().template);
        request.items[0].quantity = 4;
        let four = prepared_document(&state, &request).unwrap();
        for i in 0..3 {
            assert!(Arc::ptr_eq(&first.sources[i].image, &four.sources[i].image));
            assert!(Arc::ptr_eq(
                &first.sources[i].preview,
                &four.sources[i].preview
            ));
        }
        assert_eq!(four.sources[3].image.copy_number, Some(4));
        assert!(Arc::ptr_eq(
            &template,
            &shared.lock().unwrap().as_ref().unwrap().template
        ));
        let discarded = Arc::downgrade(&four.sources[3].image);
        request.items[0].quantity = 2;
        let two = prepared_document(&state, &request).unwrap();
        assert_eq!(shared.lock().unwrap().as_ref().unwrap().numbered.len(), 2);
        assert!(Arc::ptr_eq(&first.sources[0].image, &two.sources[0].image));
        drop(four);
        assert!(discarded.upgrade().is_none());
        request.items[0].number_copies = false;
        let plain = prepared_document(&state, &request).unwrap();
        assert!(Arc::ptr_eq(&template, &plain.sources[0].image));
        assert!(shared.lock().unwrap().as_ref().unwrap().numbered.is_empty());
        request.items[0].number_copies = true;
        let numbered_again = prepared_document(&state, &request).unwrap();
        assert_eq!(numbered_again.sources[1].image.copy_number, Some(2));
        assert!(Arc::ptr_eq(
            &template,
            &shared.lock().unwrap().as_ref().unwrap().template
        ));
    }

    #[test]
    fn preview_png_preserves_thumbnail_pixels_and_transparency() {
        let image = image::RgbaImage::from_fn(20, 40, |x, y| {
            image::Rgba([
                (x * 10) as u8,
                (y * 5) as u8,
                150,
                if x < 10 { 0 } else { 128 },
            ])
        });
        let encoded = encode_preview(&image).unwrap();
        let bytes = STANDARD
            .decode(encoded.strip_prefix("data:image/png;base64,").unwrap())
            .unwrap();
        let decoded = image::load_from_memory(&bytes).unwrap().to_rgba8();
        assert_eq!(
            decoded, image,
            "small previews should not be upscaled or altered"
        );
        let large = image::RgbaImage::from_pixel(300, 1500, image::Rgba([30, 90, 150, 128]));
        let encoded = encode_preview(&large).unwrap();
        let bytes = STANDARD
            .decode(encoded.strip_prefix("data:image/png;base64,").unwrap())
            .unwrap();
        let decoded = image::load_from_memory(&bytes).unwrap().to_rgba8();
        assert_eq!(decoded.dimensions(), (192, 960));
        assert_eq!(decoded, image::imageops::thumbnail(&large, 192, 960));
    }

    #[test]
    fn failed_preparation_does_not_retain_partially_grown_caches() {
        let state = PrintState::default();
        state
            .workspace()
            .unwrap()
            .images
            .insert(10, Arc::new(source()));
        let mut request = make_request(vec![item(10, 1, true)]);
        prepared_document(&state, &request).unwrap();
        assert!(!state.workspace().unwrap().prepared.is_empty());
        request.items[0].quantity = 5000;
        assert!(prepared_document(&state, &request).is_err());
        assert!(state.workspace().unwrap().prepared.is_empty());
    }

    #[test]
    fn removed_sources_cannot_repopulate_preparation_caches() {
        let state = PrintState::default();
        state
            .workspace()
            .unwrap()
            .images
            .insert(10, Arc::new(source()));
        let request = make_request(vec![item(10, 1, false)]);
        let originals = sources(&state, &request).unwrap();
        state.workspace().unwrap().images.remove(&10);
        assert!(prepare_sources(&state, &request, &originals).is_err());
        assert!(state.workspace().unwrap().prepared.is_empty());
    }

    #[test]
    fn application_always_allows_rotation_to_fit_the_paper() {
        let state = PrintState::default();
        state
            .workspace()
            .unwrap()
            .images
            .insert(10, Arc::new(source()));
        let mut request = make_request(vec![item(10, 1, false)]);
        request.width_mm = 130.0;
        request.height_mm = 40.0;
        let document = prepared_document(&state, &request).unwrap();
        let placement = &document.layout.pages()[0].placements[0];
        assert!(
            placement.rotated,
            "the standee can fit this paper only after a quarter-turn"
        );
        assert_eq!(document.layout.pages().len(), 1);
    }

    #[test]
    fn application_accepts_only_the_supported_size_categories() {
        let sources = vec![Arc::new(source())];
        for size in [0.5, 0.75, 1.0, 2.0, 3.0, 4.0, 5.0] {
            let mut request = make_request(vec![item(10, 1, false)]);
            request.items[0].size = size;
            assert!(prepare_test_sources(&request, &sources).is_ok());
        }
        for size in [0.0, 0.25, 1.5, 4.5, 6.0] {
            let mut request = make_request(vec![item(10, 1, false)]);
            request.items[0].size = size;
            let error = prepare_test_sources(&request, &sources).err().unwrap();
            assert_eq!(error.image_id, Some(10));
        }
    }

    #[test]
    fn preparation_reports_source_errors_and_respects_printable_paper() {
        let mut request = make_request(vec![item(10, 1, false)]);
        request.items[0].size = 3.0;
        let sources = vec![Arc::new(source())];
        let capped = prepare_test_sources(&request, &sources).unwrap();
        assert!(capped[0].image.height_mm <= 293.0);
        assert!(capped[0].image.width_mm > 73.0);
        request.height_mm = 8.0;
        request.width_mm = 8.0;
        let error = prepare_test_sources(&request, &sources).err().unwrap();
        assert_eq!(error.image_id, Some(10));
        request.items[0].size = 3.0;
        request.width_mm = 420.0;
        request.height_mm = 594.0;
        assert!(prepare_test_sources(&request, &sources).is_ok());
        request.items[0].size = 1.0;
        request.items[0].border_px = u32::MAX;
        assert!(prepare_test_sources(&request, &sources).is_err());
        request.items[0].border_px = 3;
        request.items[0].quantity = 0;
        assert!(prepare_test_sources(&request, &sources).is_err());
    }
}
