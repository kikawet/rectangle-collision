use raylib::prelude::*;

struct Box {
    id: usize,
    rec: Rectangle,
    color: Color,
    speed: Vector2,
}

impl Box {
    fn new<F>(id: usize, max_width: i32, max_height: i32, mut get_random: F) -> Self
    where
        F: FnMut(i32, i32) -> f32,
    {
        let width = 50.;
        let height = 50.;
        Box {
            id,
            rec: Rectangle {
                x: get_random(0, max_width - width as i32),
                y: get_random(0, max_height - height as i32),
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

    fn update(&mut self, delta: f32, walls: &Vec<Wall>) {
        let mut next = self.rec.clone();
        next.x = self.rec.x + self.speed.x * delta;

        //TODO: check if is so fast that went right trough
        let collided_x = walls
            .iter()
            .any(|wall| wall.check_collision_box(&self.rec, &next));

        if collided_x {
            self.speed.x *= -1.;
        }
        next = self.rec;

        next.y = self.rec.y + self.speed.y * delta;
        let collided_y = walls
            .iter()
            .any(|wall| wall.check_collision_box(&self.rec, &next));

        if collided_y {
            self.speed.y *= -1.;
        }

        self.rec.x += self.speed.x * delta;
        self.rec.y += self.speed.y * delta;
    }
}

struct Segment {
    start: Vector2,
    end: Vector2,
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
    }

    fn check_collision_box(&self, current: &Rectangle, next: &Rectangle) -> bool {
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

        let segments = vec![
            Segment {
                start: sides[0],
                end: sides[1],
            },
            Segment {
                start: sides[0],
                end: sides[2],
            },
            Segment {
                start: sides[3],
                end: sides[1],
            },
            Segment {
                start: sides[3],
                end: sides[2],
            },
        ];

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
        Wall::new(Vector2::zero(), Vector2::new(width, 0.)),
        // right
        Wall::new(Vector2::new(width, 0.), Vector2::new(width, height)),
        //bottom
        Wall::new(Vector2::new(0., height), Vector2::new(width, height)),
        //left
        Wall::new(Vector2::zero(), Vector2::new(0., height)),
    ];

    let mut boxes = (0..10)
        .into_iter()
        .map(|id| Box::new(id, width as i32, height as i32, random_generator))
        .collect::<Vec<_>>();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        boxes.iter_mut().for_each(|b| {
            b.update(dt, &walls);
        });

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::SNOW);
        boxes.iter().for_each(|b| b.draw(&mut d));
        walls.iter().for_each(|wall| wall.draw(&mut d));
    }
}
