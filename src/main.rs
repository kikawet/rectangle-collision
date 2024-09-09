use std::ops::RangeInclusive;

use collision_simulation::{
    collision::grid::Grid,
    entity::{block::Block, wall::Wall},
    traits::{Draw, Sides},
};
use raylib::prelude::*;

fn main() {
    let block_size = 15;
    let width = block_size * 16 * 4;
    let height = block_size * 9 * 4;
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Collision simulation")
        .build();
    rl.set_target_fps(60);

    let random_generator = |min, max| rl.get_random_value::<i32>(min..max).as_f32();

    let mut blocks = (0..200)
        .map(|id| Block::new_random(id, block_size, width, height, random_generator))
        .collect::<Vec<_>>();

    let widthf = width.as_f32();
    let heightf = height.as_f32();

    let walls = vec![
        //top
        Wall::new(Vector2::zero(), Vector2::new(widthf, 0.)),
        // right
        Wall::new(Vector2::new(widthf, 0.), Vector2::new(widthf, heightf)),
        //bottom
        Wall::new(Vector2::new(0., heightf), Vector2::new(widthf, heightf)),
        //left
        Wall::new(Vector2::zero(), Vector2::new(0., heightf)),
    ];

    while !rl.window_should_close() {
        // Draw
        {
            let fps = rl.get_fps();
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::SNOW);
            blocks.iter().for_each(|b| b.draw(&mut d));
            walls.iter().for_each(|wall| wall.draw(&mut d));

            d.draw_text(format!("{fps} fps").as_str(), 20, 20, 24, Color::BLACK);
        }

        // Update
        {
            let mut grid = build_grid(block_size, width, height);

            insert_items(&mut grid, &walls, &blocks);

            let collisions = blocks
                .iter()
                .map(|b| b.calculate_collisions(&walls, &blocks))
                .collect::<Vec<_>>();

            let dt = rl.get_frame_time();
            blocks
                .iter_mut()
                .zip(collisions)
                .for_each(|(b, collision)| b.update(dt, &collision));
        }
    }
}

fn insert_items<'a>(grid: &mut Grid<&'a dyn Sides>, walls: &'a [Wall], blocks: &'a [Block]) {
    for wall in walls {
        let (col_range, row_range) = calculate_ranges(wall, grid.spacing);
        grid.set_many(wall, col_range, row_range);
    }

    for block in blocks {
        let (col_range, row_range) = calculate_ranges(block, grid.spacing);
        grid.set_many(block, col_range, row_range);
    }
}

fn build_grid<'a>(block_size: i32, window_width: i32, window_height: i32) -> Grid<&'a dyn Sides> {
    let spacing = block_size * 2;
    assert_eq!(
        window_width % spacing,
        0,
        "The padding is not correct for the given screen width"
    );
    assert_eq!(
        window_height % spacing,
        0,
        "The padding is not correct for the given screen height"
    );
    let rows = (window_height / spacing).try_into().unwrap();
    let cols = (window_width / spacing).try_into().unwrap();
    let spacing = spacing.as_f32();

    Grid::new(rows, cols, spacing)
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn calculate_ranges(
    obj: &impl Sides,
    spacing: f32,
) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let top = obj.top().start / spacing;
    let bottom = obj.bottom().end / spacing;

    let col_start = top.x.floor() as usize;
    let col_end = bottom.x.floor() as usize;

    let row_start = top.y.floor() as usize;
    let row_end = bottom.y.floor() as usize;

    (col_start..=col_end, row_start..=row_end)
}
