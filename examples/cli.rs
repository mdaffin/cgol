extern crate cgol;

use cgol::{Engine, State};
use std::time::Duration;
use std::thread::sleep;

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
    println!("---------------------");
    loop {
        sleep(Duration::from_secs(1));
        engine.tick();
        println!("{}", engine);
        println!("---------------------");
    }
}
