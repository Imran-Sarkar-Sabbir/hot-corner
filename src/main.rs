mod actions;
mod calculate_display;
mod mouse_tracking;
mod utils;

fn main() {
    let corner_points = calculate_display::calculated_display_corners();
    mouse_tracking::start_tracking(corner_points);
}
