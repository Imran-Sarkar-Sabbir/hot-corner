pub trait CornerAction {
    fn go_left(&mut self);
    fn go_right(&mut self);
    fn open_window_tray(&mut self);
}
