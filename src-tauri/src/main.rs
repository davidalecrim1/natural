#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scroll;

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
};

fn build_menu(app: &tauri::App, is_natural: bool) -> tauri::Result<Menu<tauri::Wry>> {
    let status_text = if is_natural {
        "Natural Scrolling: ON"
    } else {
        "Natural Scrolling: OFF"
    };

    let status = MenuItem::with_id(app, "status", status_text, false, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let toggle = MenuItem::with_id(app, "toggle", "Toggle Scrolling", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit Natural", true, None::<&str>)?;

    Menu::with_items(app, &[&status, &sep, &toggle, &quit])
}

fn main() {
    let mut app = tauri::Builder::default()
        .setup(|app| {
            let is_natural = scroll::is_natural_scrolling();
            let menu = build_menu(app, is_natural)?;

            let icon = {
                let bytes = include_bytes!("../icons/icon.png");
                tauri::image::Image::from_bytes(bytes)?
            };

            let _tray = TrayIconBuilder::with_id("main-tray")
                .icon(icon)
                .icon_as_template(true)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .tooltip("Natural Scrolling Toggle")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => {
                        let new_state = scroll::toggle();
                        // Rebuild menu to reflect new state
                        if let Some(tray) = app.tray_by_id("main-tray") {
                            let status_text = if new_state {
                                "Natural Scrolling: ON"
                            } else {
                                "Natural Scrolling: OFF"
                            };
                            let status = MenuItem::with_id(
                                app,
                                "status",
                                status_text,
                                false,
                                None::<&str>,
                            )
                            .unwrap();
                            let sep = PredefinedMenuItem::separator(app).unwrap();
                            let toggle = MenuItem::with_id(
                                app,
                                "toggle",
                                "Toggle Scrolling",
                                true,
                                None::<&str>,
                            )
                            .unwrap();
                            let quit = MenuItem::with_id(
                                app,
                                "quit",
                                "Quit Natural",
                                true,
                                None::<&str>,
                            )
                            .unwrap();
                            if let Ok(menu) =
                                Menu::with_items(app, &[&status, &sep, &toggle, &quit])
                            {
                                let _ = tray.set_menu(Some(menu));
                            }
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error building Natural");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|_app_handle, _event| {});
}
