use raylib::prelude::*;

trait Sides {
    fn top(&self) -> Segment;
    fn right(&self) -> Segment;
    fn bottom(&self) -> Segment;
    fn left(&self) -> Segment;
}

trait Position {
    fn position(&self) -> Vector2;
    fn set_position(&mut self, new_position: Vector2);
}

impl Position for Rectangle {
    fn position(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    fn set_position(&mut self, new_position: Vector2) {
        self.x = new_position.x;
        self.y = new_position.y;
    }
}

impl Position for Box {
    fn position(&self) -> Vector2 {
        self.rec.position()
    }

    fn set_position(&mut self, new_position: Vector2) {
        self.rec.set_position(new_position);
    }
}

trait Redirect {
    fn move_up(self) -> Self;
    fn move_down(self) -> Self;
    fn move_right(self) -> Self;
    fn move_left(self) -> Self;
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

#[derive(Debug, Default)]
struct CollisionResult([Option<Vector2>; 4]);

impl CollisionResult {
    const TOP: usize = 0;
    const RIGHT: usize = 1;
    const BOTTOM: usize = 2;
    const LEFT: usize = 3;

    fn new(
        top: Option<Vector2>,
        right: Option<Vector2>,
        bottom: Option<Vector2>,
        left: Option<Vector2>,
    ) -> Self {
        let mut result = [None; 4];

        result[Self::TOP] = top;
        result[Self::RIGHT] = right;
        result[Self::BOTTOM] = bottom;
        result[Self::LEFT] = left;

        Self(result)
    }

    fn into_option(self) -> Option<Self> {
        match self.0 {
            [None, None, None, None] => None,
            _ => Some(self),
        }
    }

    fn combine(self, other: CollisionResult) -> Self {
        let combined = [
            self.0[0].or(other.0[0]),
            self.0[1].or(other.0[1]),
            self.0[2].or(other.0[2]),
            self.0[3].or(other.0[3]),
        ];

        Self(combined)
    }
}

impl FromIterator<CollisionResult> for CollisionResult {
    fn from_iter<T: IntoIterator<Item = CollisionResult>>(iter: T) -> Self {
        iter.into_iter()
            .reduce(|acc, cur| acc.combine(cur))
            .unwrap_or_default()
    }
}

struct Box {
    id: usize,
    rec: Rectangle,
    old_rec: Rectangle,
    color: Color,
    acc: Vector2,
}

impl Box {
    #[allow(dead_code)]
    fn new(rec: Rectangle, acc: Vector2) -> Self {
        Self {
            id: usize::default(),
            rec,
            old_rec: rec,
            acc,
            color: Color::BLACK,
        }
    }

    #[allow(dead_code)]
    fn new_random<F>(id: usize, max_width: i32, max_height: i32, mut get_random: F) -> Self
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

        Box {
            id,
            rec,
            old_rec: rec,
            color: Color::color_from_hsv(get_random(0, 360), 0.9, 0.9),
            acc: Vector2::new(get_random(5000, 10000), get_random(5000, 10000)) * direction,
        }
    }

    fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.draw_rectangle_rec(self.rec, self.color);
    }

    #[allow(dead_code)]
    fn draw_debug(&self, canvas: &mut RaylibDrawHandle) {
        let segments = [self.top(), self.right(), self.bottom(), self.left()];

        for Segment { start, end } in &segments {
            canvas.draw_line_v(start, end, Color::BLACK);
            canvas.draw_circle_v(start, 5., Color::RED);
            canvas.draw_circle_v(end, 5., Color::RED);
        }
    }

    fn update(&mut self, delta: f32, collided: &Option<CollisionResult>) {
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

    fn calculate_collisions(&self, walls: &[Wall], boxes: &[Box]) -> Option<CollisionResult> {
        walls
            .iter()
            .find_map(|wall| wall.check_collision_box(self))
            .or(boxes.iter().find_map(|b| b.check_collision_box(self)))
    }

    fn check_collision_box(&self, other: &Box) -> Option<CollisionResult> {
        if self.id == other.id {
            return None;
        }

        let segments = [self.top(), self.right(), self.bottom(), self.left()];

        segments
            .iter()
            .map(|segment| check_collision_segment_box(segment, other))
            .collect::<CollisionResult>()
            .into_option()
    }
}

impl Sides for Box {
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
        let normal = self.normal_with_length(10.);
        canvas.draw_circle_v(normal.start, 2., Color::FUCHSIA);
        canvas.draw_line_ex(normal.start, normal.end, 1., Color::BLACK);
    }

    fn normal_unit(&self) -> Self {
        self.normal_with_length(1.)
    }

    fn normal_with_length(&self, length: f32) -> Self {
        let delta = self.end - self.start;
        let scale = length / delta.length();
        let radius = Vector2::new(-scale * delta.y, scale * delta.x);

        let start = self.start + delta * 0.5;
        let end = start - radius;
        Segment { start, end }
    }

    fn check_collision_segment(&self, other: &Segment) -> Option<Vector2> {
        check_collision_lines(self.start, self.end, other.start, other.end)
    }

    fn angle(&self) -> f32 {
        self.start.angle_to(self.end)
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
            thick: 5.,
        }
    }

    #[allow(dead_code)]
    fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.draw_line_ex(
            self.position.start,
            self.position.end,
            self.thick,
            Color::RED,
        );
    }

    #[allow(dead_code)]
    fn draw_debug(&self, canvas: &mut RaylibDrawHandle) {
        self.get_collision_box().iter().for_each(|segment| {
            segment.draw_normals(canvas);
            segment.draw(canvas);
        });
    }

    fn check_collision_box(&self, b: &Box) -> Option<CollisionResult> {
        let segments = self.get_collision_box();

        segments
            .iter()
            .find_map(|segment| check_collision_segment_box(segment, b).into_option())
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

fn check_collision_segment_box(segment: &Segment, b: &Box) -> CollisionResult {
    let top = segment.check_collision_segment(&b.top());
    let right = segment.check_collision_segment(&b.right());
    let bottom = segment.check_collision_segment(&b.bottom());
    let left = segment.check_collision_segment(&b.left());

    CollisionResult::new(top, right, bottom, left)
}

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
        .map(|id| Box::new_random(id, width, height, random_generator))
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
