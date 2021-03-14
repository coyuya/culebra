use piston_window::*;

struct Game {
    culebra: Culebra,
}

struct Culebra {
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
}

impl Game {
    fn new() -> Self {
        Game {
            culebra: Culebra {
                pos_x: 0.0,
                pos_y: 0.0,
            },
        }
    }

    fn update(&mut self) {
        self.culebra.pos_x += 1.0;
    }

    fn draw(&self, gl: &mut G2d, ctx: &Context) {
        clear([1.0; 4], gl);
        self.culebra.draw(gl, ctx);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Culebra", [200, 200])
        .exit_on_esc(true)
        .graphics_api(OpenGL::V3_2)
        .build()
        .unwrap();

    let mut game = Game::new();

    window.set_ups(10);

    while let Some(event) = window.next() {
        // render
        if let Some(_args) = event.render_args() {
            window.draw_2d(&event, |ctx, gl, _dev| {
                game.draw(gl, &ctx);
            });
        }

        if let Some(_args) = event.update_args() {
            game.update();
        }
    }
}
