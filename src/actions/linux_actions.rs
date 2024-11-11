use rdev::{simulate, EventType, Key};

use super::actions::CornerAction;

#[cfg(target_os = "linux")]
pub struct LinuxCornerAction;

#[cfg(target_os = "linux")]
impl CornerAction for LinuxCornerAction {
    fn go_left(&self) {
        simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
        simulate(&EventType::KeyPress(Key::Alt)).unwrap();
        simulate(&EventType::KeyPress(Key::LeftArrow)).unwrap();

        simulate(&EventType::KeyRelease(Key::LeftArrow)).unwrap();
        simulate(&EventType::KeyRelease(Key::Alt)).unwrap();
        simulate(&EventType::KeyRelease(Key::ControlLeft)).unwrap();
    }

    fn go_right(&self) {
        simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
        simulate(&EventType::KeyPress(Key::Alt)).unwrap();
        simulate(&EventType::KeyPress(Key::RightArrow)).unwrap();

        simulate(&EventType::KeyRelease(Key::RightArrow)).unwrap();
        simulate(&EventType::KeyRelease(Key::Alt)).unwrap();
        simulate(&EventType::KeyRelease(Key::ControlLeft)).unwrap();
    }

    fn open_window_tray(&self) {
        simulate(&EventType::KeyPress(Key::MetaLeft)).unwrap();
        simulate(&EventType::KeyRelease(Key::MetaLeft)).unwrap();
    }
}
