use raylib::{math::Vector2, prelude::RaylibDraw};

use crate::{collision::collision_result::CollisionResult, entity::segment::Segment};

pub trait Sides {
    fn top(&self) -> Segment;
    fn right(&self) -> Segment;
    fn bottom(&self) -> Segment;
    fn left(&self) -> Segment;
}

pub trait Position {
    fn position(&self) -> Vector2;
    fn set_position(&mut self, new_position: Vector2);
}

pub trait Redirect {
    fn move_up(self) -> Self;
    fn move_down(self) -> Self;
    fn move_right(self) -> Self;
    fn move_left(self) -> Self;
}

pub trait Collision {
    fn check_collision(&self, b: &impl Sides) -> CollisionResult;
}

pub trait Draw {
    fn draw(&self, canvas: &mut impl RaylibDraw);
}
