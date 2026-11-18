mod print;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(print::PrintState::default())
        .invoke_handler(tauri::generate_handler![
            print::choose_images,
            print::import_images,
            print::remove_image,
            print::preview_layout,
            print::export_pdf,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
