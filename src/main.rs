use collision_simulation::{
    collision::grid::Grid,
    entity::{block::Block, wall::Wall},
    traits::{Draw, GridItemTrait, Sides},
};
use raylib::prelude::*;

fn main() {
    let block_size = 15;
    let width = 15 * 16 * 6;
    let height = 15 * 9 * 6;
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Collision simulation")
        .build();
    rl.set_target_fps(60);

    let random_generator = |min, max| rl.get_random_value::<i32>(min..max).as_f32();

    let mut blocks = (0..1200)
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
        Wall::new(
            Vector2::new(0. + 2., heightf - 3.),
            Vector2::new(widthf - 2., heightf - 3.),
        ),
        //left
        Wall::new(Vector2::zero(), Vector2::new(0., heightf)),
    ];

    while !rl.window_should_close() {
        let mut grid = build_grid(block_size, width, height);
        insert_items(&mut grid, &walls, &blocks);

        // Draw
        {
            let fps = rl.get_fps();
            let mut display = rl.begin_drawing(&thread);

            display.clear_background(Color::SNOW);
            blocks.iter().for_each(|b| b.draw(&mut display));
            walls.iter().for_each(|wall| wall.draw(&mut display));

            grid.draw(&mut display);

            display.draw_text(format!("{fps} fps").as_str(), 20, 20, 24, Color::BLACK);
        }

        // Update
        {
            let collisions = blocks
                .iter()
                .map(|b| b.calculate_collisions(&grid))
                .collect::<Vec<_>>();

            let dt = rl.get_frame_time();
            blocks
                .iter_mut()
                .zip(collisions)
                .for_each(|(b, collision)| b.update(dt, &collision));
        }
    }
}

fn insert_items<'a>(
    grid: &mut Grid<&'a dyn GridItemTrait>,
    walls: &'a [Wall],
    blocks: &'a [Block],
) {
    for wall in walls {
        let (row_range, col_range) = wall.calculate_grid_ranges(grid.spacing);
        grid.set_many(wall, row_range, col_range);
    }

    for block in blocks {
        let (row_range, col_range) = block.calculate_grid_ranges(grid.spacing);
        grid.set_many(block, row_range, col_range);
    }
}

fn build_grid<'a>(
    block_size: i32,
    window_width: i32,
    window_height: i32,
) -> Grid<&'a dyn GridItemTrait<'a>> {
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
