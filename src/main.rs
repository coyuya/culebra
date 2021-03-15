use piston_window::*;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Game {
    culebra: Culebra,
}

struct Culebra {
    direction: Direction,
    pos_x: f64,
    pos_y: f64,
}

impl Culebra {
    fn draw(&self, gl: &mut G2d, ctx: &Context) {
        rectangle(
            [1.0, 0.0, 0.0, 1.0], // rojo
            [self.pos_x, self.pos_y, 10.0, 10.0],
            ctx.transform,
            gl,
        );
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Up => self.pos_y -= 1.0,
            Direction::Down => self.pos_y += 1.0,
            Direction::Left => self.pos_x -= 1.0,
            Direction::Right => self.pos_x += 1.0,
        }
    }
}

impl Game {
    fn new() -> Self {
        Game {
            culebra: Culebra {
                direction: Direction::Right,
                pos_x: 0.0,
                pos_y: 0.0,
            },
        }
    }

    fn update(&mut self) {
        self.culebra.update();
    }

    fn draw(&self, gl: &mut G2d, ctx: &Context) {
        clear([1.0; 4], gl);
        self.culebra.draw(gl, ctx);
    }

    fn input(&mut self, key: &Key) {
        match key {
            Key::W => self.culebra.direction = Direction::Up,
            Key::S => self.culebra.direction = Direction::Down,
            Key::A => self.culebra.direction = Direction::Left,
            Key::D => self.culebra.direction = Direction::Right,
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

    window.set_ups(60);

    while let Some(event) = window.next() {
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
        }
    }
}
