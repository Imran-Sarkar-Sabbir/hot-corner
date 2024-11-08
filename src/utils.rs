#[derive(Debug)]
pub enum CornerResult {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Corners {
    pub top_left: Position,
    pub bottom_left: Position,
    pub top_right: Position,
    pub bottom_right: Position,
}

impl Corners {
    pub fn check_match(&self, position: &Position) -> CornerResult {
        if self.top_left.x == position.x && self.top_left.y == position.y {
            return CornerResult::TopLeft;
        } else if self.top_right.x == position.x && self.top_right.y == position.y {
            return CornerResult::TopRight;
        } else if self.bottom_left.x == position.x && self.bottom_left.y == position.y {
            return CornerResult::BottomLeft;
        } else if self.bottom_right.x == position.x && self.bottom_right.y == position.y {
            return CornerResult::BottomRight;
        }

        CornerResult::None
    }
}
