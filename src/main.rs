mod actions;
mod calculate_display;
mod mouse_tracking;
mod utils;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use utils::check_change_display_status;

fn main() {
    let (channel_send, rx) = mpsc::channel::<()>();
    let channel_recv = Arc::new(Mutex::new(rx));
    let mut current_display_count = 0;
    let mut is_running = false;
    let cloned_receiver = Arc::clone(&channel_recv);

    start(cloned_receiver);

    loop {
        if check_change_display_status(&mut current_display_count) {
            if is_running {
                channel_send.send(()).unwrap();
                thread::sleep(Duration::from_secs(10));
            }
            is_running = true;
        }
        thread::sleep(Duration::from_secs(30));
    }
}

fn start(channel_recv: Arc<Mutex<mpsc::Receiver<()>>>) {
    let points = calculate_display::calculated_display_corners();
    let corner_points = Arc::new(Mutex::new(points));
    thread::spawn(move || {
        mouse_tracking::start_tracking(corner_points.clone(), channel_recv);
    });
}
