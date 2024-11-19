mod actions;
mod calculate_display;
mod mouse_tracking;
mod utils;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let (channel_send, rx) = mpsc::channel::<()>();
    let channel_recv = Arc::new(Mutex::new(rx));
    let cloned_receiver = Arc::clone(&channel_recv);

    start(cloned_receiver);

    loop {
        thread::sleep(Duration::from_secs(40));
        channel_send.send(()).unwrap();
    }
}

fn start(channel_recv: Arc<Mutex<mpsc::Receiver<()>>>) {
    let points = calculate_display::calculated_display_corners();
    let corner_points = Arc::new(Mutex::new(points));
    thread::spawn(move || {
        mouse_tracking::start_tracking(corner_points.clone(), channel_recv);
    });
}
