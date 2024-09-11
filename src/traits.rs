use std::{fmt::Debug, ops::RangeInclusive};

use raylib::{math::Vector2, prelude::RaylibDraw};

use crate::{
    collision::{
        collision_result::CollisionResult,
        grid::{Col, Row},
    },
    entity::{block::Block, segment::Segment},
};

pub trait Sides {
    fn top(&self) -> Segment;
    fn right(&self) -> Segment;
    fn bottom(&self) -> Segment;
    fn left(&self) -> Segment;

    fn aabb(&self) -> Segment {
        let corners = [self.top(), self.right(), self.bottom(), self.left()]
            .iter()
            .flat_map(|segment| [segment.start, segment.end])
            .collect::<Vec<_>>();

        let min = corners
            .iter()
            .fold(Vector2::new(f32::MAX, f32::MAX), |acc, cur| {
                Vector2::new(acc.x.min(cur.x), acc.y.min(cur.y))
            });

        let max = corners
            .iter()
            .fold(Vector2::new(f32::MIN, f32::MIN), |acc, cur| {
                Vector2::new(acc.x.max(cur.x), acc.y.max(cur.y))
            });

        Segment {
            start: min,
            end: max,
        }
    }

    // TODO: create an alternative method that returns Iterator<(Row, Col)>.
    // A thin, large and rotated object can create a huge empty area to check in the grid.
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn calculate_grid_ranges(&self, spacing: f32) -> (RangeInclusive<Row>, RangeInclusive<Col>) {
        let aabb = self.aabb();
        let width = aabb.end.x - aabb.start.x;
        let height = aabb.end.y - aabb.start.y;

        let col_start = (aabb.start.x / spacing) as usize;
        let col_end = ((aabb.start.x + width) / spacing) as usize;

        let row_start = (aabb.start.y / spacing) as usize;
        let row_end = ((aabb.start.y + height) / spacing) as usize;

        (Row(row_start)..=Row(row_end), Col(col_start)..=Col(col_end))
    }
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

pub trait Collision<'a> {
    fn check_collision(&'a self, b: &'a dyn GridItemTrait<'a>) -> CollisionResult;
}

pub trait Draw {
    fn draw(&self, canvas: &mut impl RaylibDraw);
}

pub trait GridItemTrait<'a>: Sides + PartialEq<&'a Block> + Debug {}
