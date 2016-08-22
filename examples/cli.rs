extern crate cgol;
extern crate rustty;

use cgol::{Engine, State};
use std::time::Duration;

use rustty::{Terminal, Event};
use rustty::ui::Painter;

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut engine = Engine::new(80, 40);
    let mut paused = true;
    engine.randomise();


    'main: loop {
        let row = term.rows() - 1;
        let col = term.cols() - 7;
        while let Some(Event::Key(ch)) = term.get_event(Duration::new(0, 200)).unwrap() {
            match ch {
                'q' => break 'main,
                'p' => paused = !paused,
                'n' => {engine.tick(); break},
                'r' => engine.randomise(),
                e => term.printline(0, row - 1, &format!("{:?}", e)),
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
        term.printline(0, row, &format!("Iterations: {}    Alive: {}    Dead: {}", engine.iterations(), alive, dead));
        term.printline(0, row, &format!("Iterations: {}    Alive: {}    Dead: {}", engine.iterations(), alive, dead));
        if !paused {
            engine.tick(); 
            term.printline(col, row, "      ");
        } else {
            term.printline(col, row, "Paused");
        }
        term.swap_buffers().unwrap();
    }
}
