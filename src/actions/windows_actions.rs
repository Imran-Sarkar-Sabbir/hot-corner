#[cfg(target_os = "windows")]
use super::actions::CornerAction;

use enigo::Direction::{Click, Press, Release};
use enigo::{Enigo, Keyboard};

#[cfg(target_os = "windows")]
pub struct WindowsCornerAction {
    pub enigo: Enigo,
}

#[cfg(target_os = "windows")]
impl CornerAction for WindowsCornerAction {
    fn go_left(&mut self) {
        let _ = self.enigo.key(enigo::Key::Control, Press);
        let _ = self.enigo.key(enigo::Key::Meta, Press);
        let _ = self.enigo.key(enigo::Key::LeftArrow, Click);
        let _ = self.enigo.key(enigo::Key::Meta, Release);
        let _ = self.enigo.key(enigo::Key::Control, Release);
    }

    fn go_right(&mut self) {
        let _ = self.enigo.key(enigo::Key::Control, Press);
        let _ = self.enigo.key(enigo::Key::Meta, Press);
        let _ = self.enigo.key(enigo::Key::RightArrow, Click);
        let _ = self.enigo.key(enigo::Key::Meta, Release);
        let _ = self.enigo.key(enigo::Key::Control, Release);
    }

    fn open_window_tray(&mut self) {
        let _ = self.enigo.key(enigo::Key::Meta, Press);
        let _ = self.enigo.key(enigo::Key::Tab, Click);
        let _ = self.enigo.key(enigo::Key::Meta, Release);
    }
}
