use std::f32::consts::PI;

use raylib::prelude::*;

#[derive(Clone)]
struct Box {
    id: usize,
    rec: Rectangle,
    color: Color,
    speed: Vector2,
}

impl Box {
    fn new(rec: Rectangle, speed: Vector2) -> Self {
        Self {
            id: 0,
            rec,
            speed,
            color: Color::BLACK,
        }
    }

    fn new_random<F>(id: usize, max_width: i32, max_height: i32, mut get_random: F) -> Self
    where
        F: FnMut(i32, i32) -> f32,
    {
        let padding = 20;
        let width = 15.;
        let height = 15.;
        Box {
            id,
            rec: Rectangle {
                x: get_random(padding, max_width - width as i32 - padding),
                y: get_random(padding, max_height - height as i32 - padding),
                width,
                height,
            },
            color: Color::color_from_hsv(get_random(0, 360), 0.9, 0.9),
            speed: Vector2::new(get_random(100, 500), get_random(100, 500)),
        }
    }

    fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.draw_rectangle_rec(self.rec, self.color);
    }

    fn update(mut self, delta: f32, walls: &Vec<Wall>, boxes: &Vec<Box>) -> Self {
        let mut next = self.rec.clone();
        next.x = self.rec.x + self.speed.x * delta;

        //TODO: check if is so fast that went right trough
        let collided_x = walls
            .iter()
            .any(|wall| wall.check_collision_box(&self.rec, &next))
            || boxes
                .iter()
                .any(|b| b.id != self.id && b.rec.check_collision_recs(&next));

        if collided_x {
            self.speed.x *= -1.;
        }
        next = self.rec;

        next.y = self.rec.y + self.speed.y * delta;
        let collided_y = walls
            .iter()
            .any(|wall| wall.check_collision_box(&self.rec, &next))
            || boxes
                .iter()
                .any(|b| b.id != self.id && b.rec.check_collision_recs(&next));

        if collided_y {
            self.speed.y *= -1.;
        }

        self.rec.x += self.speed.x * delta;
        self.rec.y += self.speed.y * delta;

        self
    }
}

struct Segment {
    start: Vector2,
    end: Vector2,
}

#[allow(dead_code)]
impl Segment {
    fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.draw_line_ex(self.start, self.end, 1., Color::BLACK);
        canvas.draw_circle_v(self.start, 5., Color::RED);
    }

    fn draw_normals(&self, canvas: &mut RaylibDrawHandle) {
        let delta = self.end - self.start;
        let scale = 10. / delta.length();
        let radius = Vector2::new(-scale * delta.y, scale * delta.x);

        let normal = self.start + delta * 0.5;
        let end = normal - radius;

        canvas.draw_line_ex(normal, end, 1., Color::BLACK);
    }
}

struct Wall {
    position: Segment,
    thick: f32,
}

impl Wall {
    fn new(start: Vector2, end: Vector2) -> Self {
        Self {
            position: Segment { start, end },
            // thick: 5.,
            thick: 15.,
        }
    }

    fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.draw_line_ex(
            self.position.start,
            self.position.end,
            self.thick,
            Color::RED,
        );

        // self.get_collision_box().iter().for_each(|segment| {
        //     segment.draw_normals(canvas);
        //     segment.draw(canvas);
        // });
    }

    fn check_collision_box(&self, current: &Rectangle, next: &Rectangle) -> bool {
        let segments = self.get_collision_box();

        let connection_line = Segment {
            start: Vector2::new(current.x, current.y),
            end: Vector2::new(next.x, next.y),
        };

        segments
            .iter()
            .any(|Segment { start, end }| check_collision_line_rec(start, end, &next))
            || segments.iter().any(|Segment { start, end }| {
                check_collision_lines(start, end, connection_line.start, connection_line.end)
                    .is_some()
            })
    }

    fn get_collision_box(&self) -> Vec<Segment> {
        let delta = self.position.end - self.position.start;
        let length = delta.length();

        let scale = self.thick / (2. * length);
        let radius = Vector2::new(-scale * delta.y, scale * delta.x);

        let sides = vec![
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

fn check_collision_line_rec(start: &Vector2, end: &Vector2, rec: &Rectangle) -> bool {
    // top
    check_collision_lines(
        start,
        end,
        Vector2::new(rec.x, rec.y),
        Vector2::new(rec.x + rec.width, rec.y),
    )
    .is_some()
    //right
        || check_collision_lines(
            start,
            end,
            Vector2::new(rec.x + rec.width, rec.y),
            Vector2::new(rec.x + rec.width, rec.y + rec.height),
        )
        .is_some()
        // bottom
        || check_collision_lines(
            start,
            end,
            Vector2::new(rec.x, rec.y + rec.height),
            Vector2::new(rec.x + rec.width, rec.y + rec.height),
        )
        .is_some()
        // left
        || check_collision_lines(
            start,
            end,
            Vector2::new(rec.x, rec.y),
            Vector2::new(rec.x, rec.y + rec.height),
        )
        .is_some()
}

#[inline]
fn lerp_vec2(v0: Vector2, v1: Vector2, amount: f32) -> Vector2 {
    return v0 + ((v1 - v0) * amount);
}

fn main() {
    let width = 640.;
    let height = 480.;
    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Collision simulation")
        .build();

    let random_generator = |min, max| rl.get_random_value::<i32>(min..max).as_f32();

    let walls = vec![
        //top
        // Wall::new(Vector2::zero(), Vector2::new(width, 0.)),
        // right
        // Wall::new(Vector2::new(width, 0.), Vector2::new(width, height)),
        //bottom
        // Wall::new(Vector2::new(0., height), Vector2::new(width, height)),
        //left
        // Wall::new(Vector2::zero(), Vector2::new(0., height)),
        Wall::new(Vector2::new(20., 400.), Vector2::new(620., 400.)),
    ];

    let mut boxes = vec![Box::new(
        Rectangle {
            x: 20.,
            y: 20.,
            width: 20.,
            height: 20.,
        },
        Vector2 { x: 150., y: 250. },
    )];
    // (0..0)
    //     .into_iter()
    //     .map(|id| Box::new_random(id, width as i32, height as i32, random_generator))
    //     .collect::<Vec<_>>();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let old_boxes = boxes.clone();
        boxes = boxes
            .into_iter()
            .map(|b| b.update(dt, &walls, &old_boxes))
            .collect();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::SNOW);
        boxes.iter().for_each(|b| b.draw(&mut d));
        walls.iter().for_each(|wall| wall.draw(&mut d));
    }
}
