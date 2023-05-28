// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, NativeImage, Submenu, SystemTray, SystemTrayMenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .menu(
            Menu::new().add_submenu(Submenu::new(
                "Test",
                Menu::new()
                    .add_item(CustomMenuItem::new("Add", "Add").native_image(NativeImage::Add))
                    .add_item(
                        CustomMenuItem::new("Advanced", "Advanced")
                            .native_image(NativeImage::Advanced),
                    )
                    .add_item(
                        CustomMenuItem::new("Bluetooth", "Bluetooth")
                            .native_image(NativeImage::Bluetooth),
                    )
                    .add_item(
                        CustomMenuItem::new("Bookmarks", "Bookmarks")
                            .native_image(NativeImage::Bookmarks),
                    )
                    .add_item(
                        CustomMenuItem::new("Caution", "Caution")
                            .native_image(NativeImage::Caution),
                    )
                    .add_item(
                        CustomMenuItem::new("ColorPanel", "ColorPanel")
                            .native_image(NativeImage::ColorPanel),
                    )
                    .add_item(
                        CustomMenuItem::new("ColumnView", "ColumnView")
                            .native_image(NativeImage::ColumnView),
                    )
                    .add_item(
                        CustomMenuItem::new("Computer", "Computer")
                            .native_image(NativeImage::Computer),
                    )
                    .add_item(
                        CustomMenuItem::new("EnterFullScreen", "EnterFullScreen")
                            .native_image(NativeImage::EnterFullScreen),
                    )
                    .add_item(
                        CustomMenuItem::new("Everyone", "Everyone")
                            .native_image(NativeImage::Everyone),
                    )
                    .add_item(
                        CustomMenuItem::new("ExitFullScreen", "ExitFullScreen")
                            .native_image(NativeImage::ExitFullScreen),
                    )
                    .add_item(
                        CustomMenuItem::new("FlowView", "FlowView")
                            .native_image(NativeImage::FlowView),
                    )
                    .add_item(
                        CustomMenuItem::new("Folder", "Folder").native_image(NativeImage::Folder),
                    )
                    .add_item(
                        CustomMenuItem::new("FolderBurnable", "FolderBurnable")
                            .native_image(NativeImage::FolderBurnable),
                    )
                    .add_item(
                        CustomMenuItem::new("FolderSmart", "FolderSmart")
                            .native_image(NativeImage::FolderSmart),
                    )
                    .add_item(
                        CustomMenuItem::new("FollowLinkFreestanding", "FollowLinkFreestanding")
                            .native_image(NativeImage::FollowLinkFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("FontPanel", "FontPanel")
                            .native_image(NativeImage::FontPanel),
                    )
                    .add_item(
                        CustomMenuItem::new("GoLeft", "GoLeft").native_image(NativeImage::GoLeft),
                    )
                    .add_item(
                        CustomMenuItem::new("GoRight", "GoRight")
                            .native_image(NativeImage::GoRight),
                    )
                    .add_item(CustomMenuItem::new("Home", "Home").native_image(NativeImage::Home))
                    .add_item(
                        CustomMenuItem::new("IChatTheater", "IChatTheater")
                            .native_image(NativeImage::IChatTheater),
                    )
                    .add_item(
                        CustomMenuItem::new("IconView", "IconView")
                            .native_image(NativeImage::IconView),
                    )
                    .add_item(CustomMenuItem::new("Info", "Info").native_image(NativeImage::Info))
                    .add_item(
                        CustomMenuItem::new("InvalidDataFreestanding", "InvalidDataFreestanding")
                            .native_image(NativeImage::InvalidDataFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("LeftFacingTriangle", "LeftFacingTriangle")
                            .native_image(NativeImage::LeftFacingTriangle),
                    )
                    .add_item(
                        CustomMenuItem::new("ListView", "ListView")
                            .native_image(NativeImage::ListView),
                    )
                    .add_item(
                        CustomMenuItem::new("LockLocked", "LockLocked")
                            .native_image(NativeImage::LockLocked),
                    )
                    .add_item(
                        CustomMenuItem::new("LockUnlocked", "LockUnlocked")
                            .native_image(NativeImage::LockUnlocked),
                    )
                    .add_item(
                        CustomMenuItem::new("MenuMixedState", "MenuMixedState")
                            .native_image(NativeImage::MenuMixedState),
                    )
                    .add_item(
                        CustomMenuItem::new("MenuOnState", "MenuOnState")
                            .native_image(NativeImage::MenuOnState),
                    )
                    .add_item(
                        CustomMenuItem::new("MobileMe", "MobileMe")
                            .native_image(NativeImage::MobileMe),
                    )
                    .add_item(
                        CustomMenuItem::new("MultipleDocuments", "MultipleDocuments")
                            .native_image(NativeImage::MultipleDocuments),
                    )
                    .add_item(
                        CustomMenuItem::new("Network", "Network")
                            .native_image(NativeImage::Network),
                    )
                    .add_item(CustomMenuItem::new("Path", "Path").native_image(NativeImage::Path))
                    .add_item(
                        CustomMenuItem::new("PreferencesGeneral", "PreferencesGeneral")
                            .native_image(NativeImage::PreferencesGeneral),
                    )
                    .add_item(
                        CustomMenuItem::new("QuickLook", "QuickLook")
                            .native_image(NativeImage::QuickLook),
                    )
                    .add_item(
                        CustomMenuItem::new("RefreshFreestanding", "RefreshFreestanding")
                            .native_image(NativeImage::RefreshFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("Refresh", "Refresh")
                            .native_image(NativeImage::Refresh),
                    )
                    .add_item(
                        CustomMenuItem::new("Remove", "Remove").native_image(NativeImage::Remove),
                    )
                    .add_item(
                        CustomMenuItem::new("RevealFreestanding", "RevealFreestanding")
                            .native_image(NativeImage::RevealFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("RightFacingTriangle", "RightFacingTriangle")
                            .native_image(NativeImage::RightFacingTriangle),
                    )
                    .add_item(
                        CustomMenuItem::new("Share", "Share").native_image(NativeImage::Share),
                    )
                    .add_item(
                        CustomMenuItem::new("Slideshow", "Slideshow")
                            .native_image(NativeImage::Slideshow),
                    )
                    .add_item(
                        CustomMenuItem::new("SmartBadge", "SmartBadge")
                            .native_image(NativeImage::SmartBadge),
                    )
                    .add_item(
                        CustomMenuItem::new("StatusAvailable", "StatusAvailable")
                            .native_image(NativeImage::StatusAvailable),
                    )
                    .add_item(
                        CustomMenuItem::new("StatusNone", "StatusNone")
                            .native_image(NativeImage::StatusNone),
                    )
                    .add_item(
                        CustomMenuItem::new("StatusPartiallyAvailable", "StatusPartiallyAvailable")
                            .native_image(NativeImage::StatusPartiallyAvailable),
                    )
                    .add_item(
                        CustomMenuItem::new("StatusUnavailable", "StatusUnavailable")
                            .native_image(NativeImage::StatusUnavailable),
                    )
                    .add_item(
                        CustomMenuItem::new("StopProgressFreestanding", "StopProgressFreestanding")
                            .native_image(NativeImage::StopProgressFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("StopProgress", "StopProgress")
                            .native_image(NativeImage::StopProgress),
                    )
                    .add_item(
                        CustomMenuItem::new("TrashEmpty", "TrashEmpty")
                            .native_image(NativeImage::TrashEmpty),
                    )
                    .add_item(
                        CustomMenuItem::new("TrashFull", "TrashFull")
                            .native_image(NativeImage::TrashFull),
                    )
                    .add_item(CustomMenuItem::new("User", "User").native_image(NativeImage::User))
                    .add_item(
                        CustomMenuItem::new("UserAccounts", "UserAccounts")
                            .native_image(NativeImage::UserAccounts),
                    )
                    .add_item(
                        CustomMenuItem::new("UserGroup", "UserGroup")
                            .native_image(NativeImage::UserGroup),
                    )
                    .add_item(
                        CustomMenuItem::new("UserGuest", "UserGuest")
                            .native_image(NativeImage::UserGuest),
                    ),
            )),
        )
        .system_tray(SystemTray::new())
        .setup(|app| {
            let tray = app.tray_handle();
            tray.set_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("Add", "Add").native_image(NativeImage::Add))
                    .add_item(
                        CustomMenuItem::new("Advanced", "Advanced")
                            .native_image(NativeImage::Advanced),
                    )
                    .add_item(
                        CustomMenuItem::new("Bluetooth", "Bluetooth")
                            .native_image(NativeImage::Bluetooth),
                    )
                    .add_item(
                        CustomMenuItem::new("Bookmarks", "Bookmarks")
                            .native_image(NativeImage::Bookmarks),
                    )
                    .add_item(
                        CustomMenuItem::new("Caution", "Caution")
                            .native_image(NativeImage::Caution),
                    )
                    .add_item(
                        CustomMenuItem::new("ColorPanel", "ColorPanel")
                            .native_image(NativeImage::ColorPanel),
                    )
                    .add_item(
                        CustomMenuItem::new("ColumnView", "ColumnView")
                            .native_image(NativeImage::ColumnView),
                    )
                    .add_item(
                        CustomMenuItem::new("Computer", "Computer")
                            .native_image(NativeImage::Computer),
                    )
                    .add_item(
                        CustomMenuItem::new("EnterFullScreen", "EnterFullScreen")
                            .native_image(NativeImage::EnterFullScreen),
                    )
                    .add_item(
                        CustomMenuItem::new("Everyone", "Everyone")
                            .native_image(NativeImage::Everyone),
                    )
                    .add_item(
                        CustomMenuItem::new("ExitFullScreen", "ExitFullScreen")
                            .native_image(NativeImage::ExitFullScreen),
                    )
                    .add_item(
                        CustomMenuItem::new("FlowView", "FlowView")
                            .native_image(NativeImage::FlowView),
                    )
                    .add_item(
                        CustomMenuItem::new("Folder", "Folder").native_image(NativeImage::Folder),
                    )
                    .add_item(
                        CustomMenuItem::new("FolderBurnable", "FolderBurnable")
                            .native_image(NativeImage::FolderBurnable),
                    )
                    .add_item(
                        CustomMenuItem::new("FolderSmart", "FolderSmart")
                            .native_image(NativeImage::FolderSmart),
                    )
                    .add_item(
                        CustomMenuItem::new("FollowLinkFreestanding", "FollowLinkFreestanding")
                            .native_image(NativeImage::FollowLinkFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("FontPanel", "FontPanel")
                            .native_image(NativeImage::FontPanel),
                    )
                    .add_item(
                        CustomMenuItem::new("GoLeft", "GoLeft").native_image(NativeImage::GoLeft),
                    )
                    .add_item(
                        CustomMenuItem::new("GoRight", "GoRight")
                            .native_image(NativeImage::GoRight),
                    )
                    .add_item(CustomMenuItem::new("Home", "Home").native_image(NativeImage::Home))
                    .add_item(
                        CustomMenuItem::new("IChatTheater", "IChatTheater")
                            .native_image(NativeImage::IChatTheater),
                    )
                    .add_item(
                        CustomMenuItem::new("IconView", "IconView")
                            .native_image(NativeImage::IconView),
                    )
                    .add_item(CustomMenuItem::new("Info", "Info").native_image(NativeImage::Info))
                    .add_item(
                        CustomMenuItem::new("InvalidDataFreestanding", "InvalidDataFreestanding")
                            .native_image(NativeImage::InvalidDataFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("LeftFacingTriangle", "LeftFacingTriangle")
                            .native_image(NativeImage::LeftFacingTriangle),
                    )
                    .add_item(
                        CustomMenuItem::new("ListView", "ListView")
                            .native_image(NativeImage::ListView),
                    )
                    .add_item(
                        CustomMenuItem::new("LockLocked", "LockLocked")
                            .native_image(NativeImage::LockLocked),
                    )
                    .add_item(
                        CustomMenuItem::new("LockUnlocked", "LockUnlocked")
                            .native_image(NativeImage::LockUnlocked),
                    )
                    .add_item(
                        CustomMenuItem::new("MenuMixedState", "MenuMixedState")
                            .native_image(NativeImage::MenuMixedState),
                    )
                    .add_item(
                        CustomMenuItem::new("MenuOnState", "MenuOnState")
                            .native_image(NativeImage::MenuOnState),
                    )
                    .add_item(
                        CustomMenuItem::new("MobileMe", "MobileMe")
                            .native_image(NativeImage::MobileMe),
                    )
                    .add_item(
                        CustomMenuItem::new("MultipleDocuments", "MultipleDocuments")
                            .native_image(NativeImage::MultipleDocuments),
                    )
                    .add_item(
                        CustomMenuItem::new("Network", "Network")
                            .native_image(NativeImage::Network),
                    )
                    .add_item(CustomMenuItem::new("Path", "Path").native_image(NativeImage::Path))
                    .add_item(
                        CustomMenuItem::new("PreferencesGeneral", "PreferencesGeneral")
                            .native_image(NativeImage::PreferencesGeneral),
                    )
                    .add_item(
                        CustomMenuItem::new("QuickLook", "QuickLook")
                            .native_image(NativeImage::QuickLook),
                    )
                    .add_item(
                        CustomMenuItem::new("RefreshFreestanding", "RefreshFreestanding")
                            .native_image(NativeImage::RefreshFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("Refresh", "Refresh")
                            .native_image(NativeImage::Refresh),
                    )
                    .add_item(
                        CustomMenuItem::new("Remove", "Remove").native_image(NativeImage::Remove),
                    )
                    .add_item(
                        CustomMenuItem::new("RevealFreestanding", "RevealFreestanding")
                            .native_image(NativeImage::RevealFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("RightFacingTriangle", "RightFacingTriangle")
                            .native_image(NativeImage::RightFacingTriangle),
                    )
                    .add_item(
                        CustomMenuItem::new("Share", "Share").native_image(NativeImage::Share),
                    )
                    .add_item(
                        CustomMenuItem::new("Slideshow", "Slideshow")
                            .native_image(NativeImage::Slideshow),
                    )
                    .add_item(
                        CustomMenuItem::new("SmartBadge", "SmartBadge")
                            .native_image(NativeImage::SmartBadge),
                    )
                    .add_item(
                        CustomMenuItem::new("StatusAvailable", "StatusAvailable")
                            .native_image(NativeImage::StatusAvailable),
                    )
                    .add_item(
                        CustomMenuItem::new("StatusNone", "StatusNone")
                            .native_image(NativeImage::StatusNone),
                    )
                    .add_item(
                        CustomMenuItem::new("StatusPartiallyAvailable", "StatusPartiallyAvailable")
                            .native_image(NativeImage::StatusPartiallyAvailable),
                    )
                    .add_item(
                        CustomMenuItem::new("StatusUnavailable", "StatusUnavailable")
                            .native_image(NativeImage::StatusUnavailable),
                    )
                    .add_item(
                        CustomMenuItem::new("StopProgressFreestanding", "StopProgressFreestanding")
                            .native_image(NativeImage::StopProgressFreestanding),
                    )
                    .add_item(
                        CustomMenuItem::new("StopProgress", "StopProgress")
                            .native_image(NativeImage::StopProgress),
                    )
                    .add_item(
                        CustomMenuItem::new("TrashEmpty", "TrashEmpty")
                            .native_image(NativeImage::TrashEmpty),
                    )
                    .add_item(
                        CustomMenuItem::new("TrashFull", "TrashFull")
                            .native_image(NativeImage::TrashFull),
                    )
                    .add_item(CustomMenuItem::new("User", "User").native_image(NativeImage::User))
                    .add_item(
                        CustomMenuItem::new("UserAccounts", "UserAccounts")
                            .native_image(NativeImage::UserAccounts),
                    )
                    .add_item(
                        CustomMenuItem::new("UserGroup", "UserGroup")
                            .native_image(NativeImage::UserGroup),
                    )
                    .add_item(
                        CustomMenuItem::new("UserGuest", "UserGuest")
                            .native_image(NativeImage::UserGuest),
                    ),
            )
            .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
