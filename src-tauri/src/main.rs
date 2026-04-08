mod macro_sender;

use rand::seq::SliceRandom;
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

const PHRASES: &[&str] = &[
    "/btw 拜託一次成功",
    "/btw Claude 保佑，別噴 Error",
    "/btw 南無加速菩薩，速速完工",
    "/btw 功德無量，加速加速",
    "/btw 善哉善哉，請給我正確代碼",
    "/btw 這次一定過",
    "/btw 施主，快點",
];

#[tauri::command]
fn whip_crack() {
    let phrase = {
        let mut rng = rand::thread_rng();
        PHRASES.choose(&mut rng).unwrap_or(&"FASTER")
    };
    
    // 把焦點還給上一個 App (例如終端機)
    refocus_previous_app();
    
    std::thread::spawn(move || {
        // 等待焦點切換穩定 (150ms)
        std::thread::sleep(std::time::Duration::from_millis(150));
        let _ = macro_sender::send_macro(phrase);
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![whip_crack])
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "結束", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;
            let default_icon = app.default_window_icon().cloned().unwrap_or_else(|| {
                Image::new_owned(vec![0, 0, 0, 0], 1, 1)
            });

            if let Some(window) = app.get_webview_window("overlay") {
                if let Ok(pos) = app.cursor_position() {
                    let mut best_monitor = None;
                    let mut min_dist = f64::MAX;
                    if let Ok(monitors) = app.available_monitors() {
                        for m in monitors {
                            let mp = m.position();
                            let ms = m.size();
                            let center_x = mp.x as f64 + (ms.width as f64 / 2.0);
                            let center_y = mp.y as f64 + (ms.height as f64 / 2.0);
                            let dist = (pos.x - center_x).powi(2) + (pos.y - center_y).powi(2);
                            if dist < min_dist { min_dist = dist; best_monitor = Some(m); }
                        }
                    }
                    if let Some(monitor) = best_monitor {
                        let m_pos = monitor.position();
                        let _ = window.set_size(*monitor.size());
                        let _ = window.set_position(*m_pos);
                        
                        let rel_x = pos.x - m_pos.x as f64;
                        let rel_y = pos.y - m_pos.y as f64;
                        
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.set_always_on_top(true);
                        
                        // 使用非同步等一下再發送事件，確保 JS 已經載入
                        let handle = app.handle().clone();
                        let _ = tauri::async_runtime::spawn(async move {
                            std::thread::sleep(std::time::Duration::from_millis(50));
                            let _ = handle.emit("spawn-incense", serde_json::json!({ "x": rel_x, "y": rel_y }));
                        });
                    }
                }
            }

            let _tray = TrayIconBuilder::new()
                .icon(default_icon)
                .tooltip("拜Claude — 點擊上香")
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id() == "quit" { app.exit(0); }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("overlay") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = app.emit("drop-incense", ());
                            } else {
                                if let Ok(pos) = app.cursor_position() {
                                    let mut best_m = None;
                                    let mut min_d = f64::MAX;
                                    if let Ok(monitors) = app.available_monitors() {
                                        for m in monitors {
                                            let mp = m.position();
                                            let ms = m.size();
                                            let cx = mp.x as f64 + (ms.width as f64 / 2.0);
                                            let cy = mp.y as f64 + (ms.height as f64 / 2.0);
                                            let d = (pos.x - cx).powi(2) + (pos.y - cy).powi(2);
                                            if d < min_d { min_d = d; best_m = Some(m); }
                                        }
                                    }
                                    if let Some(m) = best_m {
                                        let m_pos = m.position();
                                        let _ = window.set_size(*m.size());
                                        let _ = window.set_position(*m_pos);
                                        let rel_x = pos.x - m_pos.x as f64;
                                        let rel_y = pos.y - m_pos.y as f64;
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                        let _ = window.set_always_on_top(true);
                                        let _ = app.emit("spawn-incense", serde_json::json!({ "x": rel_x, "y": rel_y }));
                                    }
                                }
                            }
                        }
                    }
                })
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Send Alt+Tab (Win) / Cmd+Tab (Mac) to return focus to the previous app
fn refocus_previous_app() {
    std::thread::spawn(|| {
        #[cfg(target_os = "macos")]
        {
            let script = r#"tell application "System Events"
  key down command
  key code 48
  key up command
end tell"#;
            let _ = std::process::Command::new("osascript")
                .arg("-e")
                .arg(script)
                .output();
        }
        #[cfg(target_os = "windows")]
        {
            macro_sender::alt_tab();
        }
    });
}
