use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    misc::AsF32,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    collision::collision_result::CollisionResult,
    traits::{Collision, Position, Redirect, Sides},
};

use super::{segment::Segment, wall::Wall};

pub struct Block {
    id: usize,
    rec: Rectangle,
    old_rec: Rectangle,
    color: Color,
    acc: Vector2,
}

impl Block {
    pub fn new(rec: Rectangle, acc: Vector2) -> Self {
        Self {
            id: usize::default(),
            rec,
            old_rec: rec,
            acc,
            color: Color::BLACK,
        }
    }

    pub fn new_random<F>(id: usize, max_width: i32, max_height: i32, mut get_random: F) -> Self
    where
        F: FnMut(i32, i32) -> f32,
    {
        let padding = 20;
        let width = 15;
        let height = 15;

        let rec = Rectangle {
            x: get_random(padding, max_width - width - padding),
            y: get_random(padding, max_height - height - padding),
            width: width.as_f32(),
            height: height.as_f32(),
        };

        let direction = Vector2::new(get_random(-1, 1).signum(), get_random(-1, 1).signum());

        Self {
            id,
            rec,
            old_rec: rec,
            color: Color::color_from_hsv(get_random(0, 360), 0.9, 0.9),
            acc: Vector2::new(get_random(5000, 10000), get_random(5000, 10000)) * direction,
        }
    }

    pub fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.draw_rectangle_rec(self.rec, self.color);
    }

    pub fn draw_debug(&self, canvas: &mut RaylibDrawHandle) {
        let segments = [self.top(), self.right(), self.bottom(), self.left()];

        for Segment { start, end } in &segments {
            canvas.draw_line_v(start, end, Color::BLACK);
            canvas.draw_circle_v(start, 5., Color::RED);
            canvas.draw_circle_v(end, 5., Color::RED);
        }
    }

    pub fn update(&mut self, delta: f32, collided: &Option<CollisionResult>) {
        let initial_speed = self.position() - self.old_rec.position();

        let speed = if let Some(collision) = collided {
            #[allow(clippy::match_same_arms)]
            match collision.0 {
                //Top, Right, Bottom, Left

                // No collision
                [None, None, None, None] => initial_speed,
                // One edge only
                [Some(_), None, None, None] => initial_speed.move_down(),
                [None, Some(_), None, None] => initial_speed.move_left(),
                [None, None, Some(_), None] => initial_speed.move_up(),
                [None, None, None, Some(_)] => initial_speed.move_right(),
                // Simple 2 edges
                [Some(_), Some(_), None, None] => initial_speed.move_down().move_left(),
                [Some(_), None, None, Some(_)] => initial_speed.move_down().move_right(),
                [None, Some(_), Some(_), None] => initial_speed.move_left().move_up(),
                [None, None, Some(_), Some(_)] => initial_speed.move_up().move_right(),
                // Hard 2 edges
                // TODO: use normal vector to handle this case
                [None, Some(_), None, Some(_)] => Vector2::new(initial_speed.x, -initial_speed.y),
                [Some(_), None, Some(_), None] => Vector2::new(-initial_speed.x, initial_speed.y),
                // Undefined behavior
                [None, Some(_), Some(_), Some(_)] => initial_speed,
                [Some(_), None, Some(_), Some(_)] => initial_speed,
                [Some(_), Some(_), None, Some(_)] => initial_speed,
                [Some(_), Some(_), Some(_), None] => initial_speed,
                [Some(_), Some(_), Some(_), Some(_)] => initial_speed,
            }
        } else {
            initial_speed
        };

        self.old_rec = self.rec;

        let new_position = self.position() + speed + self.acc * delta * delta;
        self.rec.set_position(new_position);
        self.acc = Vector2::zero();
    }

    pub fn calculate_collisions(
        &self,
        walls: &[Wall],
        blocks: &[Block],
    ) -> Option<CollisionResult> {
        walls
            .iter()
            .find_map(|wall| wall.check_collision(self).into_option())
            .or(blocks
                .iter()
                .filter(|b| self.id != b.id)
                .find_map(|b| b.check_collision(self).into_option()))
    }
}

impl Position for Block {
    fn position(&self) -> Vector2 {
        self.rec.position()
    }

    fn set_position(&mut self, new_position: Vector2) {
        self.rec.set_position(new_position);
    }
}

impl Sides for Block {
    #[inline]
    fn top(&self) -> Segment {
        Segment {
            start: self.position(),
            end: Vector2::new(self.position().x + self.rec.width, self.position().y),
        }
    }

    #[inline]
    fn right(&self) -> Segment {
        Segment {
            start: Vector2::new(self.position().x + self.rec.width, self.position().y),
            end: Vector2::new(
                self.position().x + self.rec.width,
                self.position().y + self.rec.height,
            ),
        }
    }

    #[inline]
    fn bottom(&self) -> Segment {
        Segment {
            start: Vector2::new(self.position().x, self.position().y + self.rec.height),
            end: Vector2::new(
                self.position().x + self.rec.width,
                self.position().y + self.rec.height,
            ),
        }
    }

    #[inline]
    fn left(&self) -> Segment {
        Segment {
            start: self.position(),
            end: Vector2::new(self.position().x, self.position().y + self.rec.height),
        }
    }
}
