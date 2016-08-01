extern crate cgol;

use cgol::{Engine, State};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut engine = Engine::new(10, 10);
    engine.randomise();
    println!("{}", engine);
    loop {
        sleep(Duration::from_millis(100));
        engine.tick();
        println!("{}", engine);
    }
}
