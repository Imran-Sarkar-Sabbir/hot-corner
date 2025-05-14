
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

#[cfg(not(target_os = "windows"))]
const CORRECTION_PIXES:i32 = 0;
#[cfg(target_os = "windows")]
const CORRECTION_PIXES:i32 = 2;

impl Corners {

    fn check_position_match(x: i32, y: i32) -> bool {
        #[cfg(not(target_os = "windows"))]
        return x == y;
        #[cfg(target_os = "windows")]
        return (x-y).abs() <= CORRECTION_PIXES;
    }

    pub fn check_match(&self, position: &Position) -> CornerResult {
        if Corners::check_position_match(self.top_left.x, position.x) && Corners::check_position_match(self.top_left.y, position.y) {
            return CornerResult::TopLeft;
        } else if Corners::check_position_match(self.top_right.x , position.x) && Corners::check_position_match(self.top_right.y, position.y ){
            return CornerResult::TopRight;
        } else if Corners::check_position_match(self.bottom_left.x, position.x) && Corners::check_position_match(self.bottom_left.y, position.y ){
            return CornerResult::BottomLeft;
        } else if Corners::check_position_match(self.bottom_right.x , position.x) && Corners::check_position_match(self.bottom_right.y, position.y) {
            return CornerResult::BottomRight;
        }
        CornerResult::None
    }

    pub fn re_assign(&mut self, new_value: Self) {
        self.top_left = new_value.top_left;
        self.top_right = new_value.top_right;
        self.bottom_left = new_value.bottom_left;
        self.bottom_right = new_value.bottom_right;
    }
}