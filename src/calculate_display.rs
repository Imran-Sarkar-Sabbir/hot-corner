use crate::utils::{Corners, Position};
use display_info::DisplayInfo;

pub fn calculated_display_corners() -> Corners {
    let display_infos = DisplayInfo::all().unwrap();
    let mut top_left = Position { x: 0, y: 0 };
    let mut top_right = Position { x: 0, y: 0 };

    let mut bottom_left = Position { x: 0, y: 0 };
    let mut bottom_right = Position { x: 0, y: 0 };

    let mut selected_display: Option<DisplayInfo> = None;

    for display_info in display_infos {
        if let Some(display) = selected_display {
            if display.width < display_info.width {
                selected_display = Some(display_info);
            } else {
                selected_display = Some(display);
            }
        } else {
            selected_display = Some(display_info);
        }
    }

    // Set corner positions based on the selected display
    if let Some(display) = selected_display {
        top_left = Position {
            x: display.x,
            y: display.y,
        };
        top_right = Position {
            x: display.x + display.width as i32 - 1,
            y: display.y,
        };
        bottom_left = Position {
            x: display.x,
            y: match display.y + display.height as i32 - 1 {
                value if value < 0 => 0,
                value => value,
            },
        };
        bottom_right = Position {
            x: display.x + display.width as i32 - 1,
            y: match display.y + display.height as i32 - 1 {
                value if value < 0 => 0,
                value => value,
            },
        };
    }

    Corners {
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    }
}
