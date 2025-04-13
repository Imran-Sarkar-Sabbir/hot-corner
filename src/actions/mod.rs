mod actions;

#[cfg(target_os = "linux")]
mod linux_actions;

#[cfg(target_os = "macos")]
mod mac_actions;

#[cfg(target_os = "windows")]
mod windows_actions;

use actions::CornerAction;

use enigo::{Enigo, Settings};
use std::cell::Cell;

#[cfg(target_os = "macos")]
use mac_actions::MacCornerAction;

#[cfg(target_os = "windows")]
use windows_actions::WindowsCornerAction;

#[cfg(target_os = "linux")]
use linux_actions::LinuxCornerAction;

static mut ACTIONS: Option<Cell<Box<dyn CornerAction>>> = None;
pub fn os_specific_corner_action() -> &'static mut Cell<Box<dyn CornerAction>> {
    unsafe {
        ACTIONS.get_or_insert_with(|| {
            #[cfg(target_os = "macos")]
            {
                Cell::new(Box::new(MacCornerAction {
                    enigo: Enigo::new(&Settings::default()).unwrap(),
                }))
            }

            #[cfg(target_os = "windows")]
            {
                Cell::new(Box::new(WindowsCornerAction {
                    enigo: Enigo::new(&Settings::default()).unwrap(),
                }))
            }

            #[cfg(target_os = "linux")]
            {
                Cell::new(Box::new(LinuxCornerAction))
            }
        })
    }
}

pub fn handle_top_left_action() {
    let actions = os_specific_corner_action().get_mut();
    actions.go_left();
}

pub fn handle_top_right_action() {
    let actions = os_specific_corner_action().get_mut();
    actions.go_right();
}

pub fn handle_bottom_left_action() {
    let actions = os_specific_corner_action().get_mut();
    actions.open_window_tray();
}

pub fn handle_bottom_right_action() {
    let actions = os_specific_corner_action().get_mut();
    actions.open_window_tray();
}
