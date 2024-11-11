use std::sync::{mpsc, Arc, Mutex};

use crate::{
    actions::{
        handle_bottom_left_action, handle_bottom_right_action, handle_top_left_action,
        handle_top_right_action,
    },
    calculate_display,
    utils::{CornerResult, Corners, Position},
};
use rdev::{listen, Event, EventType};

pub fn start_tracking(corners: Arc<Mutex<Corners>>, channel_recv: Arc<Mutex<mpsc::Receiver<()>>>) {
    let callback = move |event: Event| {
        if let Ok(_sig) = channel_recv.lock().unwrap().try_recv() {
            let new_positions = calculate_display::calculated_display_corners();
            let mut corners_value = corners.lock().unwrap();
            *corners_value = new_positions;
        }

        if let EventType::MouseMove { x, y } = event.event_type {
            let position = Position {
                x: x as i32,
                y: y as i32,
            };
            handle_mouse_event(&position, &corners.lock().unwrap());
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

static mut IS_ACTIVE: bool = false;
fn handle_mouse_event(position: &Position, corners: &Corners) {
    let result = corners.check_match(position);

    if !unsafe { IS_ACTIVE } {
        match result {
            CornerResult::TopLeft => handle_top_left_action(),
            CornerResult::TopRight => handle_top_right_action(),
            CornerResult::BottomLeft => handle_bottom_left_action(),
            CornerResult::BottomRight => handle_bottom_right_action(),
            CornerResult::None => {}
        }
    }

    match result {
        CornerResult::None => unsafe {
            IS_ACTIVE = false;
        },
        _ => unsafe {
            IS_ACTIVE = true;
        },
    }
}
