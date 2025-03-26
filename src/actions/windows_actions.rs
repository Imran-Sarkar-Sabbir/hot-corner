use rdev::{simulate, EventType, Key};

#[cfg(target_os = "windows")]
use super::actions::CornerAction;

#[cfg(target_os = "windows")]
pub struct WindowsCornerAction;

#[cfg(target_os = "windows")]
impl CornerAction for WindowsCornerAction {
    fn go_left(&mut self) {
        simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
        simulate(&EventType::KeyPress(Key::MetaLeft)).unwrap();
        simulate(&EventType::KeyPress(Key::LeftArrow)).unwrap();

        simulate(&EventType::KeyRelease(Key::LeftArrow)).unwrap();
        simulate(&EventType::KeyRelease(Key::MetaLeft)).unwrap();
        simulate(&EventType::KeyRelease(Key::ControlLeft)).unwrap();
    }

    fn go_right(&mut self) {
        simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
        simulate(&EventType::KeyPress(Key::MetaLeft)).unwrap();
        simulate(&EventType::KeyPress(Key::RightArrow)).unwrap();

        simulate(&EventType::KeyRelease(Key::RightArrow)).unwrap();
        simulate(&EventType::KeyRelease(Key::MetaLeft)).unwrap();
        simulate(&EventType::KeyRelease(Key::ControlLeft)).unwrap();
    }

    fn open_window_tray(&mut self) {
        simulate(&EventType::KeyPress(Key::MetaLeft)).unwrap();
        simulate(&EventType::KeyPress(Key::Tab)).unwrap();

        simulate(&EventType::KeyRelease(Key::Tab)).unwrap();
        simulate(&EventType::KeyRelease(Key::MetaLeft)).unwrap();
    }
}
