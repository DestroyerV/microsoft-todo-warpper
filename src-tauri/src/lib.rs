use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Error, Manager, WebviewWindow, WindowEvent,
};
use tauri_plugin_notification::NotificationExt;

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

/// Sets up autostart for the application.
fn setup_autostart(app: &tauri::App) -> Result<(), Error> {
    #[cfg(desktop)]
    {
        use tauri_plugin_autostart::MacosLauncher;
        use tauri_plugin_autostart::ManagerExt;

        app.handle().plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))?;

        let autostart_manager = app.autolaunch();
        if let Err(e) = autostart_manager.enable() {
            eprintln!("Failed to enable autostart: {}", e);
        }
        println!(
            "registered for autostart? {}",
            autostart_manager.is_enabled().unwrap_or(false)
        );
    }
    Ok(())
}

/// Sets up the system tray for the application.
fn setup_tray(app: &tauri::App) -> Result<(), Error> {
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

    TrayIconBuilder::new()
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
}

/// Sets up window events for the application.
fn setup_window_events(app: &tauri::App) {
    if let Some(window) = app.get_webview_window("main") {
        let window_clone = window.clone();
        window.on_window_event(move |event| {
            let window = window_clone.clone();
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                window.hide().unwrap();
            }
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                window
                    .set_focus()
                    .expect("Failed to set focus to the main window");
            }
        }))
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                inject_custom_css(&window).expect("Failed to inject custom CSS");
            }

            setup_autostart(app).expect("Failed to setup autostart");
            setup_tray(app).expect("Failed to setup tray");
            setup_window_events(app);
            app.notification()
                .builder()
                .title("Tauri")
                .body("Tauri is awesome")
                .show()
                .unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
