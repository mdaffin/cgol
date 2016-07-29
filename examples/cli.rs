extern crate cgol;

use cgol::{Engine, State};

fn main() {
    let mut engine = Engine::new(10, 5);
    for y in 0..5 {
        engine.set(0, y, State::Alive);
        engine.set(9, y, State::Alive);
    }
    for x in 0..10 {
        engine.set(x, 0, State::Alive);
        engine.set(x, 4, State::Alive);
    }
    println!("---------------------");
    println!("{}", engine);
    engine.tick();
    println!("---------------------");
    println!("{}", engine);
    println!("---------------------");
}
