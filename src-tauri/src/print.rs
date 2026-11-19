use std::{
    collections::HashMap,
    io::{Cursor, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
};

use base64::{Engine, engine::general_purpose::STANDARD};
use image::{DynamicImage, ImageDecoder, ImageFormat, ImageReader, Limits};
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
    preview: Option<(PrintRequest, Arc<PreparedDocument>)>,
}

struct PreparedSource {
    image_id: u32,
    copy_index: usize,
    quantity: usize,
    image: PreparedImage,
    preview: String,
}

struct PreparedDocument {
    sources: Vec<PreparedSource>,
    layout: DocumentLayout,
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
    let orientation = decoder.orientation().map_err(|error| error.to_string())?;
    let mut image = DynamicImage::from_decoder(decoder).map_err(|error| error.to_string())?;
    image.apply_orientation(orientation);

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
    workspace.preview = None;
    Ok(())
}

#[derive(Deserialize, PartialEq)]
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

#[derive(Deserialize, PartialEq)]
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

#[derive(Clone, Copy, Deserialize, PartialEq)]
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

fn prepare_sources(
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
            number_copies: item.number_copies,
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
        let copies = image_manipulation::prepare_copies(source.as_ref(), count, &options)
            .map_err(source_error)?;
        for (copy_index, copy) in copies.enumerate() {
            let image = copy.map_err(source_error)?;
            total_pixels += u64::from(image.image.width()) * u64::from(image.image.height());
            if total_pixels > 64 * 1024 * 1024 {
                return Err(PrintError {
                    message: "These standees need too much memory. Reduce copies or DPI.".into(),
                    image_id: Some(item.id),
                });
            }
            let mut bytes = Cursor::new(Vec::new());
            DynamicImage::ImageRgba8(image::imageops::thumbnail(&image.image, 480, 960))
                .write_to(&mut bytes, ImageFormat::Png)
                .map_err(|error| PrintError::new(error.to_string()))?;
            prepared.push(PreparedSource {
                image_id: item.id,
                copy_index,
                quantity: if item.number_copies { 1 } else { item.quantity },
                image,
                preview: format!(
                    "data:image/png;base64,{}",
                    STANDARD.encode(bytes.into_inner())
                ),
            });
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
    let sources = prepare_sources(request, &sources(state, request)?)?;
    let layout = image_packing::layout(&items(&sources), &request.options())?;
    Ok(Arc::new(PreparedDocument { sources, layout }))
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
        let preview = document_preview(&document);
        state.workspace()?.preview = Some((request, document));
        Ok(preview)
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
            preview: source.preview.clone(),
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
        let items = items(&document.sources);
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
        save_pdf(&path, &items, &document.layout)?;
        Ok(Some(path.to_string_lossy().into_owned()))
    })
    .await
}

fn save_pdf(
    path: &Path,
    items: &[ImageItem<'_, image::RgbaImage>],
    layout: &DocumentLayout,
) -> Result<(), PrintError> {
    let pdf = image_packing::render_pdf(items, layout)?;
    let folder = path
        .parent()
        .ok_or_else(|| PrintError::new("Choose a folder for the PDF."))?;
    let save_error =
        |error: std::io::Error| PrintError::new(format!("Could not save the PDF. {error}"));
    let mut file = tempfile::NamedTempFile::new_in(folder).map_err(save_error)?;
    file.write_all(&pdf).map_err(save_error)?;
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
        save_pdf(&destination, &packing_items, &document.layout).unwrap();
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
        let prepared = prepare_sources(&request, &sources).unwrap();
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
            assert!(prepare_sources(&request, &sources).is_ok());
        }
        for size in [0.0, 0.25, 1.5, 4.5, 6.0] {
            let mut request = make_request(vec![item(10, 1, false)]);
            request.items[0].size = size;
            let error = prepare_sources(&request, &sources).err().unwrap();
            assert_eq!(error.image_id, Some(10));
        }
    }

    #[test]
    fn preparation_reports_source_errors_and_respects_printable_paper() {
        let mut request = make_request(vec![item(10, 1, false)]);
        request.items[0].size = 3.0;
        let sources = vec![Arc::new(source())];
        let capped = prepare_sources(&request, &sources).unwrap();
        assert!(capped[0].image.height_mm <= 293.0);
        assert!(capped[0].image.width_mm > 73.0);
        request.height_mm = 8.0;
        request.width_mm = 8.0;
        let error = prepare_sources(&request, &sources).err().unwrap();
        assert_eq!(error.image_id, Some(10));
        request.items[0].size = 3.0;
        request.width_mm = 420.0;
        request.height_mm = 594.0;
        assert!(prepare_sources(&request, &sources).is_ok());
        request.items[0].size = 1.0;
        request.items[0].border_px = u32::MAX;
        assert!(prepare_sources(&request, &sources).is_err());
        request.items[0].border_px = 3;
        request.items[0].quantity = 0;
        assert!(prepare_sources(&request, &sources).is_err());
    }
}
