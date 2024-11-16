use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                // Inject CSS after the WebView is ready
                let _ = window.eval(
                    r#"
                    console.log("Injecting custom CSS...");
                    
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
                    "#,
                );
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
