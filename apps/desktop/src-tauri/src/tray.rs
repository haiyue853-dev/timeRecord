use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

const OPEN_MAIN_MENU_ID: &str = "tray-open-main";
const QUIT_APP_MENU_ID: &str = "tray-quit-app";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayMenuAction {
    OpenMainWindow,
    QuitApp,
}

pub fn parse_tray_menu_action(id: &str) -> Option<TrayMenuAction> {
    match id {
        OPEN_MAIN_MENU_ID => Some(TrayMenuAction::OpenMainWindow),
        QUIT_APP_MENU_ID => Some(TrayMenuAction::QuitApp),
        _ => None,
    }
}

pub fn create_tray_icon<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let icon = app.default_window_icon().cloned().ok_or_else(|| {
        tauri::Error::AssetNotFound("default window icon is missing".to_string())
    })?;
    let open_item = MenuItem::with_id(app, OPEN_MAIN_MENU_ID, "打开主页面", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, QUIT_APP_MENU_ID, "完全退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open_item, &quit_item])?;

    TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            if let Some(action) = parse_tray_menu_action(event.id().as_ref()) {
                handle_tray_menu_action(app, action);
            }
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            }
            | TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } => {
                toggle_main_window(&tray.app_handle());
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

fn handle_tray_menu_action<R: Runtime>(app: &AppHandle<R>, action: TrayMenuAction) {
    match action {
        TrayMenuAction::OpenMainWindow => show_main_window(app),
        TrayMenuAction::QuitApp => app.exit(0),
    }
}

fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

pub fn toggle_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            show_main_window(app);
        }
    }
}

pub fn hide_to_tray<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}
