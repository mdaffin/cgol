extern crate cgol;

use cgol::{Engine, State};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut engine = Engine::random(150, 10);
    engine.set(1, 0, State::Alive);
    engine.set(2, 1, State::Alive);
    engine.set(0, 2, State::Alive);
    engine.set(1, 2, State::Alive);
    engine.set(2, 2, State::Alive);

    println!("{}", engine);
    loop {
        sleep(Duration::from_millis(100));
        engine.tick();
        println!("{}", engine);
    }
}
