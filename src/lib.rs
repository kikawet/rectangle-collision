use collision::collision_result::CollisionResult;
use raylib::math::{Rectangle, Vector2};

use traits::{Collision, GridItemTrait, Position, Redirect, Sides};

pub mod collision;
pub mod entity;

#[allow(clippy::return_self_not_must_use)]
pub mod traits;

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

impl<'a, T: Sides> Collision<'a> for T {
    fn check_collision(&'a self, other: &'a dyn GridItemTrait<'a>) -> CollisionResult {
        let segments = [self.top(), self.right(), self.bottom(), self.left()];
        let other_segments = [other.top(), other.right(), other.bottom(), other.left()];

        other_segments
            .iter()
            .map(|other| {
                segments
                    .iter()
                    .map(|segment| segment.check_collision_segment(other))
                    .collect::<CollisionResult>()
            })
            .collect::<CollisionResult>()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests_collision_simulation {
    use entity::segment::Segment;
    use raylib::{color::Color, texture::Image};
    use traits::Position;
    use traits::Sides;

    use super::*;

    fn draw_rec<T: Sides>(obj: &T, canvas: &mut Image) {
        let segments = [obj.top(), obj.right(), obj.bottom(), obj.left()];

        for Segment { start, end } in &segments {
            canvas.draw_line_v(start, end, Color::BLACK);
            canvas.draw_circle_v(start, 5, Color::RED);
            canvas.draw_circle_v(end, 5, Color::RED);
        }
    }

    fn draw_index<T: Position>(obj: &T, index: usize, canvas: &mut Image) {
        #[allow(clippy::cast_possible_truncation)]
        canvas.draw_text(
            format!("{}", index + 1).as_str(),
            obj.position().x as i32 + 5,
            obj.position().y as i32 - 20,
            24,
            Color::BLACK,
        );
    }

    fn render_objects<T: Sides + Position>(objects: &[T]) {
        let width = 640;
        let height = 480;

        let (rl, thread) = raylib::init().size(width, height).build();

        let mut image = rl.load_image_from_screen(&thread);
        image.clear_background(Color::SNOW);
        objects.iter().enumerate().for_each(|(index, obj)| {
            draw_rec(obj, &mut image);
            draw_index(obj, index, &mut image);
        });

        image.export_image("render_test_collision_simulation.png");
        // image.export_image(format!("{test_name}.png").as_str());
    }

    #[test]
    fn test_check_collision_with_no_collision() {
        let rec1 = Rectangle::new(10., 10., 5., 5.);
        let rec2 = Rectangle::new(20., 20., 5., 5.);

        let collision = rec1.check_collision(&rec2);

        assert!(collision.into_option().is_none());
    }

    #[test]
    fn test_check_collision_with_top_right_corner() {
        let rec1 = Rectangle::new(50., 100., 100., 100.);
        let rec2 = Rectangle::new(100., 50., 100., 100.);

        let collision = rec1.check_collision(&rec2);

        assert!(collision.clone().into_option().is_some());
        assert!(collision.0[CollisionResult::TOP].is_some());
        assert!(collision.0[CollisionResult::RIGHT].is_some());
        assert!(collision.0[CollisionResult::BOTTOM].is_none());
        assert!(collision.0[CollisionResult::LEFT].is_none());
    }

    #[test]
    fn test_check_collision_with_bottom_left_corner() {
        let rec1 = Rectangle::new(50., 100., 100., 100.);
        let rec2 = Rectangle::new(100., 50., 100., 100.);

        let collision = rec2.check_collision(&rec1);

        assert!(collision.clone().into_option().is_some());
        assert!(collision.0[CollisionResult::TOP].is_none());
        assert!(collision.0[CollisionResult::RIGHT].is_none());
        assert!(collision.0[CollisionResult::BOTTOM].is_some());
        assert!(collision.0[CollisionResult::LEFT].is_some());
    }
}
