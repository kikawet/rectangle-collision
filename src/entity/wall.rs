use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::traits::Sides;

use super::segment::Segment;

pub struct Wall {
    position: Segment,
    thick: f32,
}

impl Wall {
    #[must_use]
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

    fn get_collision_box(&self) -> [Segment; 4] {
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

        [
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

impl Sides for Wall {
    fn top(&self) -> Segment {
        let [top, _, _, _] = self.get_collision_box();
        top
    }

    fn right(&self) -> Segment {
        let [_, right, _, _] = self.get_collision_box();
        right
    }

    fn bottom(&self) -> Segment {
        let [_, _, bottom, _] = self.get_collision_box();
        bottom
    }

    fn left(&self) -> Segment {
        let [_, _, _, left] = self.get_collision_box();
        left
    }
}
