use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Error, Manager, WebviewWindow,
};

/// Injects custom CSS into the specified Tauri webview window.
fn inject_custom_css(window: &WebviewWindow) -> Result<(), Error> {
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
                if let Err(e) = window.set_focus() {
                    eprintln!("Failed to set focus to the main window: {}", e);
                }
            }
        }))
        // Plugin to manage persisted scope
        .plugin(tauri_plugin_persisted_scope::init())
        // Plugin to handle shell commands
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Inject custom CSS into the main window
            if let Some(window) = app.get_webview_window("main") {
                if let Err(e) = inject_custom_css(&window) {
                    eprintln!("Failed to inject custom CSS: {}", e);
                }
            }

            // Autostart plugin
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                if let Err(e) = app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    Some(vec!["--flag1", "--flag2"]),
                )) {
                    eprintln!("Failed to initialize autostart plugin: {}", e);
                }

                // Get the autostart manager
                let autostart_manager = app.autolaunch();
                // Enable autostart
                if let Err(e) = autostart_manager.enable() {
                    eprintln!("Failed to enable autostart: {}", e);
                }
                // Check enable state
                match autostart_manager.is_enabled() {
                    Ok(enabled) => println!("Registered for autostart? {}", enabled),
                    Err(e) => eprintln!("Failed to check autostart status: {}", e),
                }
            }

            // Tray Icon plugin
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                window.set_focus().unwrap();
                            } else {
                                window.show().unwrap();
                            }
                        } else {
                            eprintln!("Main window not found");
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {
                        eprintln!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
