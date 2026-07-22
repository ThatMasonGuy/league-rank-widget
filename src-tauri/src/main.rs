#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Manager, PhysicalPosition,
};
#[cfg(target_os = "windows")]
use window_vibrancy::{apply_acrylic, apply_blur};
#[cfg(target_os = "windows")]
use windows_sys::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::{CreateRoundRectRgn, DeleteObject, SetWindowRgn},
    UI::{HiDpi::GetDpiForWindow, WindowsAndMessaging::GetClientRect},
};

const WINDOW_CORNER_RADIUS: i32 = 17;

#[cfg(target_os = "windows")]
fn apply_rounded_window_region<R: tauri::Runtime>(window: &tauri::WebviewWindow<R>) {
    let Ok(handle) = window.window_handle() else {
        eprintln!("Unable to access the native window handle for rounded corners");
        return;
    };
    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
        return;
    };

    let hwnd = handle.hwnd.get() as HWND;
    let mut client_rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    unsafe {
        if GetClientRect(hwnd, &mut client_rect) == 0 {
            eprintln!("Unable to measure the native window for rounded corners");
            return;
        }

        let dpi = GetDpiForWindow(hwnd) as i32;
        let radius = (WINDOW_CORNER_RADIUS * dpi + 95) / 96;
        let region = CreateRoundRectRgn(
            0,
            0,
            client_rect.right + 1,
            client_rect.bottom + 1,
            radius * 2,
            radius * 2,
        );

        if region.is_null() {
            eprintln!("Unable to create the native rounded-corner region");
            return;
        }

        // Windows owns the region after a successful call; clean it up only
        // when the region could not be assigned to the window.
        if SetWindowRgn(hwnd, region, 1) == 0 {
            let _ = DeleteObject(region);
            eprintln!("Unable to apply the native rounded-corner region");
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let league_tracker =
                MenuItem::with_id(app, "league_tracker", "League Tracker", false, None::<&str>)?;

            let separator = PredefinedMenuItem::separator(app)?;

            let exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;

            let tray_menu = Menu::with_items(app, &[&league_tracker, &separator, &exit])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "exit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "windows")]
                {
                    // These are configured in tauri.conf.json too. Setting them at
                    // runtime prevents Windows from restoring a native title bar or
                    // its compositor shadow while the window is being shown.
                    window.set_decorations(false)?;
                    window.set_shadow(false)?;
                }

                if let Some(monitor) = window.current_monitor().unwrap() {
                    let size = monitor.size();
                    let x = size.width - 630;
                    let y = 10;
                    window.set_position(PhysicalPosition::new(x, y))?;
                }

                window.show()?;

                #[cfg(target_os = "windows")]
                {
                    // Tauri applies native window state asynchronously. Queue the
                    // region and acrylic after the window has been shown so Windows
                    // cannot replace them while creating the frame.
                    let effects_window = window.clone();
                    window.run_on_main_thread(move || {
                        apply_rounded_window_region(&effects_window);

                        if let Err(acrylic_error) =
                            apply_acrylic(&effects_window, Some((9, 16, 24, 48)))
                        {
                            eprintln!("Unable to enable native window acrylic: {acrylic_error}");
                            if let Err(blur_error) =
                                apply_blur(&effects_window, Some((9, 16, 24, 48)))
                            {
                                eprintln!(
                                    "Unable to enable native window blur fallback: {blur_error}"
                                );
                            }
                        }
                    })?;
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
