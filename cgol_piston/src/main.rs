extern crate piston_window;
extern crate find_folder;
extern crate cgol;

use piston_window::*;
use cgol::Engine;

// struct Cell {
//     width: f64,
//     height: f64,
// }
//
// impl Cell {
//     pub fn render() {}
// }

struct App {
    engine: Engine,
}

impl App {
    pub fn new(width: usize, height: usize) -> App {
        App { engine: Engine::new(width, height) }
    }
    pub fn render(&self, c: &Context, g: &mut G2d) {
        let cell_width = c.viewport.unwrap().rect[2] as f64 / self.engine.width() as f64;
        let cell_height = c.viewport.unwrap().rect[3] as f64 / self.engine.height() as f64;
        for x in 0..self.engine.width() {
            for y in 0..self.engine.height() {
                let rect = Rectangle::new_border([1.0, 1.0, 1.0, 1.0], 0.5);
                let c = c.trans(x as f64 * cell_width, y as f64 * cell_height);
                rect.draw([0.0, 0.0, cell_width, cell_height],
                          &c.draw_state,
                          c.transform,
                          g)
            }
        }
    }
    pub fn update(&mut self, _args: &UpdateArgs) {}
    pub fn key_press(&mut self, _key: &Key) {}
    pub fn key_release(&mut self, _key: &Key) {}
}

fn main() {
    let (width, height) = (640, 480);
    let mut window: PistonWindow = WindowSettings::new("Conway's Game Of Life", [width, height])
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    // let assets = find_folder::Search::ParentsThenKids(3, 3)
    //     .for_folder("assets")
    //     .unwrap();
    // let ref font = assets.join("FiraSans-Regular.ttf");
    // let factory = window.factory.clone();
    // let mut glyphs = Glyphs::new(font, factory).unwrap();

    let mut app = App::new(50, 50);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
            app.render(&c, gl);
        });

        if let Some(uargs) = e.update_args() {
            app.update(&uargs);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.key_press(&key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.key_release(&key);
        }
    }
}
