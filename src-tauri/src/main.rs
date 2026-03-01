#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scroll;

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_notification::NotificationExt;

const TRAY_ID: &str = "main-tray";
const ITEM_STATUS: &str = "status";
const ITEM_TOGGLE: &str = "toggle";
const ITEM_AUTOSTART: &str = "autostart";
const ITEM_QUIT: &str = "quit";

fn scroll_label(is_natural: bool) -> &'static str {
    if is_natural {
        "Natural Scrolling: ON"
    } else {
        "Natural Scrolling: OFF"
    }
}

fn autostart_label(is_enabled: bool) -> &'static str {
    if is_enabled {
        "Launch at Login: ON"
    } else {
        "Launch at Login: OFF"
    }
}

fn build_menu<M: Manager<tauri::Wry>>(app: &M) -> tauri::Result<Menu<tauri::Wry>> {
    let status_text = scroll_label(scroll::is_natural_scrolling());
    let autostart_text = autostart_label(app.autolaunch().is_enabled().unwrap_or(false));

    let status = MenuItem::with_id(app, ITEM_STATUS, status_text, false, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let toggle = MenuItem::with_id(app, ITEM_TOGGLE, "Toggle Scrolling", true, None::<&str>)?;
    let autostart = MenuItem::with_id(app, ITEM_AUTOSTART, autostart_text, true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, ITEM_QUIT, "Quit Natural", true, None::<&str>)?;

    Menu::with_items(app, &[&status, &sep, &toggle, &autostart, &sep2, &quit])
}

fn rebuild_tray_menu(app: &AppHandle) {
    if let Ok(menu) = build_menu(app) {
        if let Some(tray) = app.tray_by_id(TRAY_ID) {
            let _ = tray.set_menu(Some(menu));
        }
    }
}

fn toggle_and_update_menu(app: &AppHandle) {
    let new_state = scroll::toggle();
    rebuild_tray_menu(app);

    let _ = app
        .notification()
        .builder()
        .title("Natural Scrolling")
        .body(scroll_label(new_state))
        .show();
}

fn toggle_autostart(app: &AppHandle) {
    let manager = app.autolaunch();
    let is_enabled = manager.is_enabled().unwrap_or(false);
    if is_enabled {
        let _ = manager.disable();
    } else {
        let _ = manager.enable();
    }
    rebuild_tray_menu(app);
}

fn main() {
    let mut app = tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
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
            let menu = build_menu(app)?;

            let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/icon.png"))?;

            let _tray = TrayIconBuilder::with_id(TRAY_ID)
                .icon(icon)
                .icon_as_template(true)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .tooltip("Natural Scrolling Toggle")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    ITEM_TOGGLE => toggle_and_update_menu(app),
                    ITEM_AUTOSTART => toggle_autostart(app),
                    ITEM_QUIT => app.exit(0),
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
