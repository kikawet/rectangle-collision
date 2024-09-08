use raylib::math::{Rectangle, Vector2};

use traits::{Position, Redirect};

pub mod collision;
pub mod entity;
mod traits;

impl Position for Rectangle {
    fn position(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    fn set_position(&mut self, new_position: Vector2) {
        self.x = new_position.x;
        self.y = new_position.y;
    }
}
impl Redirect for Vector2 {
    fn move_up(self) -> Self {
        Vector2::new(self.x, self.y.abs() * -1.)
    }

    fn move_down(self) -> Self {
        Vector2::new(self.x, self.y.abs())
    }

    fn move_right(self) -> Self {
        Vector2::new(self.x.abs(), self.y)
    }

    fn move_left(self) -> Self {
        Vector2::new(self.x.abs() * -1., self.y)
    }
}
