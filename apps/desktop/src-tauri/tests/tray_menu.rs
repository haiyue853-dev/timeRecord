use timerecord_lib::tray::{parse_tray_menu_action, TrayMenuAction};

#[test]
fn parses_supported_tray_menu_actions() {
    assert_eq!(
        parse_tray_menu_action("tray-open-main"),
        Some(TrayMenuAction::OpenMainWindow)
    );
    assert_eq!(
        parse_tray_menu_action("tray-quit-app"),
        Some(TrayMenuAction::QuitApp)
    );
    assert_eq!(parse_tray_menu_action("unknown"), None);
}
