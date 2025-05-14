use crate::{
    actions::{
        handle_bottom_left_action, handle_bottom_right_action, handle_top_left_action,
        handle_top_right_action,
    },
    calculate_display,
    utils::{CornerResult, Corners, Position},
};
use rdev::{listen, Event, EventType};
use std::time::{Duration, Instant};

pub fn start_tracking() {
    let mut corners = calculate_display::calculated_display_corners();
    let callback = move |event: Event| {
        re_calculate_display(&mut corners);
        if let EventType::MouseMove { x, y } = event.event_type {
            let position = Position {
                x: x as i32,
                y: y as i32,
            };
            handle_mouse_event(&position, &corners);
        }
    };

    listen(callback).expect("Error listening on mouse tracking listener")
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

const RECHECK_TIME: u64 = 30;
static mut CHECK_TIME: Option<Instant> = None;
fn re_calculate_display(corners: &mut Corners) {
    let now = Instant::now();
    let is_time_exceed = unsafe {
        match CHECK_TIME {
            None => {
                CHECK_TIME = Some(Instant::now() + Duration::from_secs(RECHECK_TIME));
            }
            _ => {}
        };
        CHECK_TIME.unwrap().cmp(&&now).is_lt()
    };

    if is_time_exceed {
        corners.re_assign(calculate_display::calculated_display_corners());
        unsafe {
            CHECK_TIME = Some(now + Duration::from_secs(RECHECK_TIME));
        }
    }
}
