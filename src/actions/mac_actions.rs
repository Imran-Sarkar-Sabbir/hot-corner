#[cfg(target_os = "macos")]
use super::actions::CornerAction;

use enigo::Direction::{Click, Press, Release};
use enigo::{Enigo, Key, Keyboard};

#[cfg(target_os = "macos")]
pub struct MacCornerAction {
    pub enigo: Enigo
}

#[cfg(target_os = "macos")]
impl CornerAction for MacCornerAction {
    fn go_left(&mut self) {
        let _ = self.enigo.key(Key::Control, Press);
        let _ = self.enigo.key(Key::LeftArrow, Click);
        let _ = self.enigo.key(Key::Control, Release);
    }

    fn go_right(&mut self) {
        let _ = self.enigo.key(Key::Control, Press);
        let _ = self.enigo.key(Key::RightArrow, Click);
        let _ = self.enigo.key(Key::Control, Release);
    }

    fn open_window_tray(&mut self) {
        let _ = self.enigo.key(Key::Control, Press);
        let _ = self.enigo.key(Key::UpArrow, Click);
        let _ = self.enigo.key(Key::Control, Release);
    }
}
