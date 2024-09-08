use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::collision::collision_result::CollisionResult;

use super::{block::Block, segment::Segment};

pub struct Wall {
    position: Segment,
    thick: f32,
}

impl Wall {
    pub fn new(start: Vector2, end: Vector2) -> Self {
        Self {
            position: Segment { start, end },
            thick: 5.,
        }
    }

    pub fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.draw_line_ex(
            self.position.start,
            self.position.end,
            self.thick,
            Color::RED,
        );
    }

    pub fn draw_debug(&self, canvas: &mut RaylibDrawHandle) {
        self.get_collision_box().iter().for_each(|segment| {
            segment.draw_debug(canvas);
            segment.draw(canvas);
        });
    }

    pub fn check_collision_box(&self, b: &Block) -> Option<CollisionResult> {
        let segments = self.get_collision_box();

        segments
            .iter()
            .find_map(|segment| segment.check_collision_segment_box(b).into_option())
    }

    fn get_collision_box(&self) -> Vec<Segment> {
        let delta = self.position.end - self.position.start;
        let length = delta.length();

        let scale = self.thick / (2. * length);
        let radius = Vector2::new(-scale * delta.y, scale * delta.x);

        let sides = [
            self.position.start - radius,
            self.position.start + radius,
            self.position.end - radius,
            self.position.end + radius,
        ];

        vec![
            Segment {
                start: sides[0],
                end: sides[2],
            },
            Segment {
                start: sides[2],
                end: sides[3],
            },
            Segment {
                start: sides[3],
                end: sides[1],
            },
            Segment {
                start: sides[1],
                end: sides[0],
            },
        ]
    }
}
