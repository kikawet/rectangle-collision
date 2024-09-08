use raylib::{
    check_collision_lines,
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{collision::collision_result::CollisionResult, traits::Sides};

use super::block::Block;

pub struct Segment {
    pub start: Vector2,
    pub end: Vector2,
}

impl Segment {
    pub fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.draw_line_ex(self.start, self.end, 1., Color::BLACK);
        canvas.draw_circle_v(self.start, 5., Color::RED);
    }

    pub fn draw_debug(&self, canvas: &mut RaylibDrawHandle) {
        let normal = self.normal_with_length(10.);
        canvas.draw_circle_v(normal.start, 2., Color::FUCHSIA);
        canvas.draw_line_ex(normal.start, normal.end, 1., Color::BLACK);
    }

    pub fn normal_unit(&self) -> Self {
        self.normal_with_length(1.)
    }

    fn normal_with_length(&self, length: f32) -> Self {
        let delta = self.end - self.start;
        let scale = length / delta.length();
        let radius = Vector2::new(-scale * delta.y, scale * delta.x);

        let start = self.start + delta * 0.5;
        let end = start - radius;
        Segment { start, end }
    }

    fn check_collision_segment(&self, other: &Segment) -> Option<Vector2> {
        check_collision_lines(self.start, self.end, other.start, other.end)
    }

    pub fn check_collision_segment_box(&self, b: &Block) -> CollisionResult {
        let top = self.check_collision_segment(&b.top());
        let right = self.check_collision_segment(&b.right());
        let bottom = self.check_collision_segment(&b.bottom());
        let left = self.check_collision_segment(&b.left());

        CollisionResult::new(top, right, bottom, left)
    }

    pub fn angle(&self) -> f32 {
        self.start.angle_to(self.end)
    }
}
