use crate::{
    actions::{
        handle_bottom_left_action, handle_bottom_right_action, handle_top_left_action,
        handle_top_right_action,
    },
    utils::{Corners, CornersResult, Position},
};
use rdev::{listen, Event, EventType};

pub fn start_tracking(corners: Corners) {
    let callback = move |event: Event| {
        if let EventType::MouseMove { x, y } = event.event_type {
            let position = Position {
                x: x as i32,
                y: y as i32,
            };
            handle_mouse_event(&position, &corners);
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
            CornersResult::TopLeft => handle_top_left_action(),
            CornersResult::TopRight => handle_top_right_action(),
            CornersResult::BottomLeft => handle_bottom_left_action(),
            CornersResult::BottomRight => handle_bottom_right_action(),
            CornersResult::None => {}
        }
    }

    match result {
        CornersResult::None => unsafe {
            IS_ACTIVE = false;
        },
        _ => unsafe {
            IS_ACTIVE = true;
        },
    }
}
