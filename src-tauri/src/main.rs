#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scroll;

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_notification::NotificationExt;

fn rebuild_tray_menu(app: &AppHandle) {
    let is_natural = scroll::is_natural_scrolling();
    let status_text = if is_natural {
        "Natural Scrolling: ON"
    } else {
        "Natural Scrolling: OFF"
    };

    let Ok(status) = MenuItem::with_id(app, "status", status_text, false, None::<&str>) else {
        return;
    };
    let Ok(sep) = PredefinedMenuItem::separator(app) else {
        return;
    };
    let Ok(toggle) = MenuItem::with_id(app, "toggle", "Toggle Scrolling", true, None::<&str>)
    else {
        return;
    };
    let Ok(quit) = MenuItem::with_id(app, "quit", "Quit Natural", true, None::<&str>) else {
        return;
    };

    if let Ok(menu) = Menu::with_items(app, &[&status, &sep, &toggle, &quit]) {
        if let Some(tray) = app.tray_by_id("main-tray") {
            let _ = tray.set_menu(Some(menu));
        }
    }
}

fn toggle_and_update_menu(app: &AppHandle) {
    let new_state = scroll::toggle();
    rebuild_tray_menu(app);

    let body = if new_state {
        "Natural Scrolling: ON"
    } else {
        "Natural Scrolling: OFF"
    };
    let _ = app
        .notification()
        .builder()
        .title("Natural Scrolling")
        .body(body)
        .show();
}

fn main() {
    let mut app = tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        toggle_and_update_menu(app);
                    }
                })
                .build(),
        )
        .setup(|app| {
            let is_natural = scroll::is_natural_scrolling();
            let status_text = if is_natural {
                "Natural Scrolling: ON"
            } else {
                "Natural Scrolling: OFF"
            };

            let status = MenuItem::with_id(app, "status", status_text, false, None::<&str>)?;
            let sep = PredefinedMenuItem::separator(app)?;
            let toggle =
                MenuItem::with_id(app, "toggle", "Toggle Scrolling", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit Natural", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&status, &sep, &toggle, &quit])?;

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
                    "toggle" => toggle_and_update_menu(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            // Cmd+Ctrl+N global shortcut
            let shortcut = Shortcut::new(Some(Modifiers::META | Modifiers::CONTROL), Code::KeyN);
            app.global_shortcut().register(shortcut)?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error building Natural");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|_app_handle, _event| {});
}
