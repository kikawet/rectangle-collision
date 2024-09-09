use raylib::{
    check_collision_lines,
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::traits::Draw;

#[derive(Debug)]
pub struct Segment {
    pub start: Vector2,
    pub end: Vector2,
}

impl Segment {
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
        Self { start, end }
    }

    pub fn check_collision_segment(&self, other: &Self) -> Option<Vector2> {
        check_collision_lines(self.start, self.end, other.start, other.end)
    }

    pub fn angle(&self) -> f32 {
        self.start.angle_to(self.end)
    }
}

impl Draw for Segment {
    fn draw(&self, canvas: &mut impl RaylibDraw) {
        canvas.draw_line_ex(self.start, self.end, 1., Color::BLACK);
        canvas.draw_circle_v(self.start, 5., Color::RED);
    }
}
