use tauri::Manager;
fn dummy(app: &tauri::App) {
    if let Ok(pos) = app.app_handle().cursor_position() {
        println!("{:?}", pos);
    }
}
