extern crate cgol;
extern crate rustty;

use cgol::{Engine, State};
use std::time::Duration;
use std::thread::sleep;

use rustty::{Terminal, Event};
use rustty::ui::Painter;

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut engine = Engine::new(80, 40);
    engine.randomise();


    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(Duration::new(0, 200)).unwrap() {
            match ch {
                'q' => break 'main,
                _ => {}
            }
        }
        let mut alive: u32 = 0;
        let mut dead: u32 = 0;
        for (x, y, state) in engine.iter() {
            if state == State::Alive {
                alive += 1;
            } else {
                dead += 1;
            }
            term.printline(x, y, &format!("{}", state));
        }
        let row = term.rows() - 1;
        term.printline(0, row, &format!("Iterations: {}    Alive: {}    Dead: {}", engine.iterations(), alive, dead));
        engine.tick(); 
        term.swap_buffers().unwrap();
    }
}
