use crate::{
    actions::{
        handle_bottom_left_action, handle_bottom_right_action, handle_top_left_action,
        handle_top_right_action,
    },
    utils::{CornerResult, Corners, Position},
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
