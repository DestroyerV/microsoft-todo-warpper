use tauri::Manager;

/// Injects custom CSS into the specified Tauri webview window.
fn inject_custom_css(window: &tauri::WebviewWindow) -> Result<(), tauri::Error> {
    let css_script = r#"
        const style = document.createElement('style');
        style.textContent = `
            /* Example: Hide specific elements */
            #O365_HeaderLeftRegion {
                display: none !important;
            }
            .sidebar-footer {
                display: none !important;
            }

            /* Example: Change background color */
            body {
                background-color: #f0f0f0 !important;
            }

            /* Example: Modify font styles */
            * {
                font-family: Arial, sans-serif !important;
            }
        `;
        document.head.appendChild(style);
    "#;

    window.eval(css_script)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugin to manage window state
        .plugin(tauri_plugin_window_state::Builder::new().build())
        // Plugin to handle single instance of the application
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        // Plugin to manage persisted scope
        .plugin(tauri_plugin_persisted_scope::init())
        // Plugin to handle shell commands
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                inject_custom_css(&window)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
