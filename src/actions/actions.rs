pub trait CornerAction {
    fn go_left(&self);
    fn go_right(&self);
    fn open_window_tray(&self);
}
