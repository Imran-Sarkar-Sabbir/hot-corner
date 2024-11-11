mod actions;
mod linux_actions;
mod mac_actions;
mod windows_actions;

use actions::CornerAction;

#[cfg(target_os = "macos")]
use mac_actions::MacCornerAction;

#[cfg(target_os = "windows")]
use windows_actions::WindowsCornerAction;

#[cfg(target_os = "linux")]
use linux_actions::LinuxCornerAction;

pub fn os_specific_corner_action() -> Box<dyn CornerAction> {
    #[cfg(target_os = "macos")]
    {
        Box::new(MacCornerAction)
    }

    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsCornerAction)
    }

    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxCornerAction)
    }
}

pub fn handle_top_left_action() {
    let actions = os_specific_corner_action();
    actions.go_left();
}

pub fn handle_top_right_action() {
    let actions = os_specific_corner_action();
    actions.go_right();
}

pub fn handle_bottom_left_action() {
    let actions = os_specific_corner_action();
    actions.open_window_tray();
}

pub fn handle_bottom_right_action() {
    let actions = os_specific_corner_action();
    actions.open_window_tray();
}
