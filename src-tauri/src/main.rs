#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(debug_assertions))]
use std::{fs, io::ErrorKind, path::PathBuf};

use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Manager, PhysicalPosition,
};
#[cfg(not(debug_assertions))]
use tauri::{AppHandle, Runtime};
#[cfg(not(debug_assertions))]
use tauri_plugin_autostart::ManagerExt as AutostartManagerExt;

const AUTOSTART_MENU_ID: &str = "launch_at_sign_in";
#[cfg(not(debug_assertions))]
const AUTOSTART_PREFERENCE_FILE: &str = "autostart-preference";

#[cfg(not(debug_assertions))]
fn autostart_preference_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .map(|directory| directory.join(AUTOSTART_PREFERENCE_FILE))
        .map_err(|error| format!("could not locate the app configuration directory: {error}"))
}

#[cfg(not(debug_assertions))]
fn read_autostart_preference<R: Runtime>(app: &AppHandle<R>) -> Result<Option<bool>, String> {
    let path = autostart_preference_path(app)?;

    match fs::read_to_string(path) {
        Ok(value) => match value.trim() {
            "enabled" => Ok(Some(true)),
            "disabled" => Ok(Some(false)),
            _ => Ok(None),
        },
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("could not read the autostart preference: {error}")),
    }
}

#[cfg(not(debug_assertions))]
fn write_autostart_preference<R: Runtime>(app: &AppHandle<R>, enabled: bool) -> Result<(), String> {
    let path = autostart_preference_path(app)?;
    let directory = path
        .parent()
        .ok_or_else(|| "autostart preference has no parent directory".to_string())?;

    fs::create_dir_all(directory)
        .map_err(|error| format!("could not create the app configuration directory: {error}"))?;
    fs::write(path, if enabled { "enabled\n" } else { "disabled\n" })
        .map_err(|error| format!("could not save the autostart preference: {error}"))
}

#[cfg(not(debug_assertions))]
fn autostart_is_enabled<R: Runtime>(app: &AppHandle<R>) -> Result<bool, String> {
    app.autolaunch()
        .is_enabled()
        .map_err(|error| format!("could not read the Windows autostart setting: {error}"))
}

#[cfg(not(debug_assertions))]
fn set_autostart_enabled<R: Runtime>(app: &AppHandle<R>, enabled: bool) -> Result<(), String> {
    let autostart = app.autolaunch();
    let result = if enabled {
        autostart.enable()
    } else {
        autostart.disable()
    };

    result.map_err(|error| format!("could not update the Windows autostart setting: {error}"))
}

#[cfg(not(debug_assertions))]
fn configure_autostart<R: Runtime>(app: &AppHandle<R>) -> bool {
    let preference = match read_autostart_preference(app) {
        Ok(preference) => preference,
        Err(error) => {
            eprintln!("{error}");
            return false;
        }
    };

    let should_be_enabled = preference.unwrap_or(true);
    let registered = match autostart_is_enabled(app) {
        Ok(registered) => registered,
        Err(error) => {
            eprintln!("{error}");
            return false;
        }
    };

    if registered != should_be_enabled {
        if let Err(error) = set_autostart_enabled(app, should_be_enabled) {
            eprintln!("{error}");
            return registered;
        }
    }

    if preference.is_none() {
        if let Err(error) = write_autostart_preference(app, true) {
            eprintln!("{error}");
        }
    }

    autostart_is_enabled(app).unwrap_or(should_be_enabled)
}

#[cfg(not(debug_assertions))]
fn toggle_autostart<R: Runtime>(app: &AppHandle<R>, menu_item: &CheckMenuItem<R>) {
    let enabled = match autostart_is_enabled(app) {
        Ok(enabled) => enabled,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };
    let should_be_enabled = !enabled;

    match set_autostart_enabled(app, should_be_enabled) {
        Ok(()) => {
            if let Err(error) = write_autostart_preference(app, should_be_enabled) {
                eprintln!("{error}");
            }
            if let Err(error) = menu_item.set_checked(should_be_enabled) {
                eprintln!("could not update the autostart menu item: {error}");
            }
        }
        Err(error) => {
            eprintln!("{error}");
            if let Err(error) = menu_item.set_checked(enabled) {
                eprintln!("could not restore the autostart menu item: {error}");
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            #[cfg(not(debug_assertions))]
            let autostart_enabled = configure_autostart(app.handle());
            #[cfg(debug_assertions)]
            let autostart_enabled = false;

            let league_tracker =
                MenuItem::with_id(app, "league_tracker", "League Tracker", false, None::<&str>)?;

            let launch_at_sign_in = CheckMenuItem::with_id(
                app,
                AUTOSTART_MENU_ID,
                "Launch at sign-in",
                !cfg!(debug_assertions),
                autostart_enabled,
                None::<&str>,
            )?;
            #[cfg(not(debug_assertions))]
            let launch_at_sign_in_for_handler = launch_at_sign_in.clone();

            let separator = PredefinedMenuItem::separator(app)?;

            let exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;

            let tray_menu = Menu::with_items(
                app,
                &[&league_tracker, &launch_at_sign_in, &separator, &exit],
            )?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    AUTOSTART_MENU_ID => {
                        #[cfg(not(debug_assertions))]
                        toggle_autostart(app, &launch_at_sign_in_for_handler);
                    }
                    "exit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                if let Some(monitor) = window.current_monitor().unwrap() {
                    let size = monitor.size();
                    let x = size.width - 630;
                    let y = 10;
                    window.set_position(PhysicalPosition::new(x, y))?;
                }

                window.show()?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
