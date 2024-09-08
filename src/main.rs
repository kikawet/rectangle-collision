use collision_simulation::entity::{block::Block, wall::Wall};
use raylib::prelude::*;

fn main() {
    let width = 640;
    let height = 480;
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Collision simulation")
        .build();
    rl.set_target_fps(60);

    let random_generator = |min, max| rl.get_random_value::<i32>(min..max).as_f32();

    let mut boxes = (0..200)
        .map(|id| Block::new_random(id, width, height, random_generator))
        .collect::<Vec<_>>();

    let width = width.as_f32();
    let height = height.as_f32();

    let walls = vec![
        //top
        Wall::new(Vector2::zero(), Vector2::new(width, 0.)),
        // right
        Wall::new(Vector2::new(width, 0.), Vector2::new(width, height)),
        //bottom
        Wall::new(Vector2::new(0., height), Vector2::new(width, height)),
        //left
        Wall::new(Vector2::zero(), Vector2::new(0., height)),
    ];

    while !rl.window_should_close() {
        // Draw
        {
            let fps = rl.get_fps();
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::SNOW);
            boxes.iter().for_each(|b| b.draw(&mut d));
            walls.iter().for_each(|wall| wall.draw(&mut d));

            d.draw_text(format!("{fps} fps").as_str(), 20, 20, 24, Color::BLACK);
        }

        // Update
        {
            let collisions = boxes
                .iter()
                .map(|b| b.calculate_collisions(&walls, &boxes))
                .collect::<Vec<_>>();

            let dt = rl.get_frame_time();
            boxes
                .iter_mut()
                .zip(collisions)
                .for_each(|(b, collision)| b.update(dt, &collision));
        }
    }
}
