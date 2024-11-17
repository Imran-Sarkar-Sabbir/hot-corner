#[cfg(target_os = "macos")]
use super::actions::CornerAction;

#[cfg(target_os = "windows")]
pub struct WindowsCornerAction;

#[cfg(target_os = "windows")]
impl CornerAction for WindowsCornerAction {
    fn go_left(&self) {
        println!("windows: Moving left.");
        // windows-specific logic here
    }

    fn go_right(&self) {
        println!("windows: Moving right.");
        // windows-specific logic here
    }

    fn open_window_tray(&self) {
        println!("windows: Opening window tray.");
        // windows-specific logic here
    }
}
