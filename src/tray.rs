use std::sync::Arc;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIcon, TrayIconBuilder,
};
use crate::resources;

pub fn setup_tray() -> TrayIcon {
    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("Quit", true, None);
    tray_menu.append(&quit_item).unwrap();

    let icon_data = resources::get_svg("resources/home.svg")
        .expect("Failed to load tray icon");

    let icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Material App")
        .with_icon(icon_data.as_bytes().to_vec())
        .build()
        .unwrap();

    let tray_icon = Arc::new(icon);
    let tray_icon_clone = tray_icon.clone();

    MenuEvent::subscribe(move |event| {
        if event.id == quit_item.id() {
            std::process::exit(0);
        }
    });

    (*tray_icon_clone).clone()
} 