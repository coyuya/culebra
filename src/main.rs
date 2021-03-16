use piston_window::*;
use rand::prelude::*;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    running: bool,
    culebra: Culebra,
    fruit: Fruit,
}

struct Position {
    x: f64,
    y: f64,
}

struct Culebra {
    direction: Direction,
    body: Vec<Position>,
}

struct Fruit {
    position: Position,
}

impl Fruit {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position: Position {
                x: (rng.gen_range(1..19) * 10) as f64,
                y: (rng.gen_range(1..19) * 10) as f64,
            },
        }
    }

    fn draw(&self, gl: &mut G2d, ctx: &Context) {
        rectangle(
            [0.0, 0.0, 1.0, 1.0], // blue
            [self.position.x, self.position.y, 10.0, 10.0],
            ctx.transform,
            gl,
        );
    }
}

impl Culebra {
    fn draw(&self, gl: &mut G2d, ctx: &Context) {
        // snake head
        rectangle(
            [1.0, 0.0, 0.0, 1.0], // red
            [self.body[0].x, self.body[0].y, 10.0, 10.0],
            ctx.transform,
            gl,
        );
        // snake body
        let mut body_iter = self.body.iter();
        body_iter.next();
        for part in body_iter {
            rectangle(
                [0.0, 1.0, 0.0, 1.0], // green
                [part.x, part.y, 10.0, 10.0],
                ctx.transform,
                gl,
            );
        }
    }

    fn update(&mut self) {
        // body
        if self.body.len() > 1 {
            for i in (1..self.body.len()).rev() {
                self.body[i].x = self.body[i - 1].x;
                self.body[i].y = self.body[i - 1].y;
            }
        }
        // head
        match self.direction {
            Direction::Up => self.body[0].y -= 10.0,
            Direction::Down => self.body[0].y += 10.0,
            Direction::Left => self.body[0].x -= 10.0,
            Direction::Right => self.body[0].x += 10.0,
        }
    }

    fn collision_game_over(&mut self) -> bool {
        let mut iter_body = self.body.iter();
        iter_body.next();
        if self.body[0].y > 190.0
            || self.body[0].y < 0.0
            || self.body[0].x > 190.0
            || self.body[0].x < 0.0
            || iter_body.any(|Position { x, y }| self.body[0].x == *x && self.body[0].y == *y)
        {
            return true;
        }

        false
    }

    fn collision_with(&self, pos: &Position) -> bool {
        if self.body[0].x == pos.x && self.body[0].y == pos.y {
            return true;
        }

        false
    }

    fn add_body(&mut self) {
        let mut x = self.body.last().unwrap().x;
        let mut y = self.body.last().unwrap().y;
        match self.direction {
            Direction::Up => y += 10.0,
            Direction::Down => y -= 10.0,
            Direction::Left => x += 10.0,
            Direction::Right => x -= 10.0,
        }
        self.body.push(Position { x: x, y: y });
    }
}

impl Game {
    fn new() -> Self {
        Game {
            running: true,
            culebra: Culebra {
                direction: Direction::Right,
                body: vec![Position { x: 0.0, y: 0.0 }],
            },
            fruit: Fruit::new(),
        }
    }

    fn update(&mut self) {
        self.culebra.update();
        // collision
        if self.culebra.collision_game_over() {
            self.running = false;
        }
        // fruit
        if self.culebra.collision_with(&self.fruit.position) {
            self.fruit = Fruit::new();
            self.culebra.add_body();
        }
    }

    fn draw(&self, gl: &mut G2d, ctx: &Context) {
        clear([0.0; 4], gl);
        self.culebra.draw(gl, ctx);
        self.fruit.draw(gl, ctx);
    }

    fn input(&mut self, key: &Key) {
        match key {
            Key::W if self.culebra.direction != Direction::Down => {
                self.culebra.direction = Direction::Up
            }
            Key::S if self.culebra.direction != Direction::Up => {
                self.culebra.direction = Direction::Down
            }
            Key::A if self.culebra.direction != Direction::Right => {
                self.culebra.direction = Direction::Left
            }
            Key::D if self.culebra.direction != Direction::Left => {
                self.culebra.direction = Direction::Right
            }
            Key::P => self.culebra.add_body(),
            _ => {}
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Culebra", [200, 200])
        .exit_on_esc(true)
        .graphics_api(OpenGL::V3_2)
        .build()
        .unwrap();

    let mut game = Game::new();

    window.set_ups(8);

    while let Some(event) = window.next() {
        if !game.running {
            break;
        }

        // render
        if let Some(_args) = event.render_args() {
            window.draw_2d(&event, |ctx, gl, _dev| {
                game.draw(gl, &ctx);
            });
        }

        // input
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.input(&key);
        }

        // update
        if let Some(_args) = event.update_args() {
            game.update();
            /*for i in 0..game.culebra.body.len() {
                println!("[{}] x: {} -- y: {}", i, game.culebra.body[i].x, game.culebra.body[i].y);
            }*/
        }
    }
}
