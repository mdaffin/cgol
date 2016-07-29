#![allow(dead_code)] // for now; remove later when we start using the module.

/// Simulation engine for conway's game of life.
///
/// 1. Any live cell with fewer than two live neighbours dies, as if caused by under-population.
/// 2. Any live cell with two or three live neighbours lives on to the next generation.
/// 3. Any live cell with more than three live neighbours dies, as if by over-population.
/// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

use std::ops::IndexMut;
use std::fmt;

/// The state of a cell. It can either be Dead or Alive.
#[derive(Clone, Copy, Debug)]
pub enum State {
    Alive,
    Dead,
}

/// The core engine holding the state of all the cells.
pub struct Engine {
    cells: Vec<State>,
    width: usize,
    height: usize,
}

impl Engine {
    // Creates a new engine with all cells in the `dead` state.
    pub fn new(width: usize, height: usize) -> Engine {
        Engine {
            cells: vec![State::Dead; width * height],
            width: width,
            height: height,
        }
    }

    /// Advance the engine by one tick. The game state will be updated according to the following rules:
    pub fn tick(&mut self) {
        let next = vec![State::Dead; self.cells.len()];

        self.cells = next
    }

    /// Gets the width of the world grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height of the world grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the state of the cell at (x, y).
    pub fn get(&self, x: usize, y: usize) -> State {
        self.cells[x + self.width * y]
    }

    /// Set the state of the cell at (x, y) to v.
    pub fn set(&mut self, x: usize, y: usize, v: State) {
        self.cells[x + self.width * y] = v
    }

    /// Get a mutable reference to the state of the cell at (x, y).
    pub fn get_mut<'a>(&'a mut self, (x, y): (usize, usize)) -> &'a mut State {
        self.cells.index_mut(x + self.width * y)
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                try!(write!(f, "{}", self.get(x, y)));
            }
            if y != self.height - 1 {
                try!(write!(f, "\n"));
            }
        }
        Ok(())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Alive => write!(f, "#"),
            State::Dead => write!(f, " "),
        }
    }
}
