use std::{
    collections::HashMap,
    io::{Cursor, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
};

use base64::{Engine, engine::general_purpose::STANDARD};
use image::{DynamicImage, ImageDecoder, ImageFormat, ImageReader, Limits};
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
    preview: Option<(PrintRequest, Arc<DocumentLayout>)>,
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
    allow_rotation: bool,
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
            allow_rotation: self.allow_rotation,
        }
    }
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct PrintItem {
    id: u32,
    quantity: usize,
    width_mm: f64,
    height_mm: Option<f64>,
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

fn items<'a>(
    request: &PrintRequest,
    sources: &'a [Arc<DynamicImage>],
) -> Vec<ImageItem<'a, DynamicImage>> {
    request
        .items
        .iter()
        .zip(sources)
        .map(|(item, image)| {
            let source = ImageItem::new(item.id.to_string(), image.as_ref(), item.quantity);
            match item.height_mm {
                Some(height) => source.with_size_mm(item.width_mm, height),
                None => source.with_width_mm(item.width_mm),
            }
        })
        .collect()
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
) -> Result<Vec<PreviewPage>, PrintError> {
    let state = state.inner().clone();
    blocking(move || {
        let sources = sources(&state, &request)?;
        let layout = image_packing::layout(&items(&request, &sources), &request.options())?;
        let pages = layout
            .pages()
            .iter()
            .map(|page| PreviewPage {
                width_mm: page.size.width_mm,
                height_mm: page.size.height_mm,
                placements: page
                    .placements
                    .iter()
                    .map(|placement| PreviewPlacement {
                        image_id: request.items[placement.source_index].id,
                        copy_index: placement.copy_index,
                        x_mm: placement.x_mm,
                        y_mm: placement.y_mm,
                        width_mm: placement.width_mm,
                        height_mm: placement.height_mm,
                        rotated: placement.rotated,
                    })
                    .collect(),
            })
            .collect();
        state.workspace()?.preview = Some((request, Arc::new(layout)));
        Ok(pages)
    })
    .await
}

#[tauri::command]
pub(crate) async fn export_pdf(
    app: AppHandle,
    state: State<'_, PrintState>,
    request: PrintRequest,
) -> Result<Option<String>, PrintError> {
    let state = state.inner().clone();
    blocking(move || {
        let sources = sources(&state, &request)?;
        let items = items(&request, &sources);
        let cached = state
            .workspace()?
            .preview
            .as_ref()
            .filter(|(input, _)| input == &request)
            .map(|(_, layout)| Arc::clone(layout));
        let layout = match cached {
            Some(layout) => layout,
            None => Arc::new(image_packing::layout(&items, &request.options())?),
        };
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
        save_pdf(&path, &items, &layout)?;
        Ok(Some(path.to_string_lossy().into_owned()))
    })
    .await
}

fn save_pdf(
    path: &Path,
    items: &[ImageItem<'_, DynamicImage>],
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

    #[test]
    fn imported_image_can_be_packed_and_saved_without_touching_the_source() {
        let folder = tempfile::tempdir().unwrap();
        let source = folder.path().join("token.png");
        DynamicImage::new_rgba8(600, 300).save(&source).unwrap();
        let original = std::fs::read(&source).unwrap();
        let state = PrintState::default();
        let imported = load_images(&state, vec![source.clone()]).unwrap();
        assert!(imported.errors.is_empty());
        let image = &imported.images[0];
        assert_eq!((image.width, image.height), (600, 300));
        assert!(image.preview.starts_with("data:image/png;base64,"));

        let request = PrintRequest {
            items: vec![PrintItem {
                id: image.id,
                quantity: 2,
                width_mm: 50.8,
                height_mm: Some(30.0),
            }],
            width_mm: 210.0,
            height_mm: 297.0,
            dpi: 300.0,
            margin_mm: 5.0,
            spacing_mm: 2.0,
            allow_rotation: false,
        };
        let sources = sources(&state, &request).unwrap();
        let items = items(&request, &sources);
        let layout = image_packing::layout(&items, &request.options()).unwrap();
        assert_eq!(layout.pages()[0].placements.len(), 2);
        assert_eq!(layout.pages()[0].placements[0].height_mm, 30.0);
        let destination = folder.path().join("sheet.pdf");
        std::fs::write(&destination, b"previous file").unwrap();
        save_pdf(&destination, &items, &layout).unwrap();
        assert!(std::fs::read(destination).unwrap().starts_with(b"%PDF-"));
        assert_eq!(std::fs::read(source).unwrap(), original);
    }
}
