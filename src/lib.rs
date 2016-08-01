/// Simulation engine for conway's game of life.
///
/// 1. Any live cell with fewer than two live neighbours dies, as if caused by under-population.
/// 2. Any live cell with two or three live neighbours lives on to the next generation.
/// 3. Any live cell with more than three live neighbours dies, as if by over-population.
/// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
extern crate rand;

use std::ops::IndexMut;
use std::fmt;

#[cfg(test)]
mod tests;

/// The state of a cell. It can either be Dead or Alive.
#[derive(Clone, Copy, Debug, PartialEq)]
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
    /// Creates a new engine with all cells in the `Dead` state.
    pub fn new(width: usize, height: usize) -> Engine {
        Engine {
            cells: vec![State::Dead; width * height],
            width: width,
            height: height,
        }
    }

    /// Randomises the state of all the cells.
    pub fn randomise(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = match rand::random::<bool>() {
                true => State::Alive,
                false => State::Dead,
            }
        }
    }

    /// Advance the engine by one tick.
    pub fn tick(&mut self) {
        let mut next = vec![State::Dead; self.cells.len()];
        for (x, y, state) in self.iter() {
            match self.neighbour_iter(x, y).filter(|&(_, _, c)| c == State::Alive).count() {
                a if a < 2 => next[self.to_index(x, y)] = State::Dead,
                2 => next[self.to_index(x, y)] = state,
                3 => next[self.to_index(x, y)] = State::Alive,
                a if a > 3 => next[self.to_index(x, y)] = State::Dead,
                _ => unreachable!(),
            }
        }
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
        self.cells[self.to_index(x, y)]
    }

    /// Set the state of the cell at (x, y) to v.
    pub fn set(&mut self, x: usize, y: usize, v: State) {
        let i = self.to_index(x, y);
        self.cells[i] = v;
    }

    /// Get a mutable reference to the state of the cell at (x, y).
    pub fn get_mut<'a>(&'a mut self, (x, y): (usize, usize)) -> &'a mut State {
        let i = self.to_index(x, y);
        self.cells.index_mut(i)
    }

    /// Returns an iterator over the cells.
    pub fn iter(&self) -> CellIterator {
        CellIterator {
            x: 0,
            y: 0,
            engine: &self,
        }
    }

    /// Returns an iterator over a cells neighbour cells.
    pub fn neighbour_iter(&self, x: usize, y: usize) -> NeighbourIterator {
        NeighbourIterator {
            x: x,
            y: y,
            i: 0,
            engine: &self,
        }
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        x + self.width * y
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

// An interator over the neighbours of a cell.
pub struct NeighbourIterator<'a> {
    x: usize,
    y: usize,
    i: usize,
    engine: &'a Engine,
}

impl<'a> Iterator for NeighbourIterator<'a> {
    type Item = (usize, usize, State);
    fn next(&mut self) -> Option<(usize, usize, State)> {
        let w = self.engine.width - 1;
        let h = self.engine.height - 1;

        while self.i < 8 {
            let i = self.i;
            self.i += 1;
            let (x, y) = match i {
                0 if self.x > 0 && self.y > 0 => (self.x - 1, self.y - 1),
                1 if self.y > 0 => (self.x, self.y - 1),
                2 if self.x < w && self.y > 0 => (self.x + 1, self.y - 1),
                3 if self.x > 0 => (self.x - 1, self.y),
                4 if self.x < w => (self.x + 1, self.y),
                5 if self.x > 0 && self.y < h => (self.x - 1, self.y + 1),
                6 if self.y < h => (self.x, self.y + 1),
                7 if self.x < w && self.y < h => (self.x + 1, self.y + 1),
                i if i >= 8 => unreachable!(),
                _ => continue,
            };
            return Some((x, y, self.engine.get(x, y)));
        }
        None
    }
}

/// An iterator over all the cells in an Engine.
pub struct CellIterator<'a> {
    x: usize,
    y: usize,
    engine: &'a Engine,
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = (usize, usize, State);
    fn next(&mut self) -> Option<(usize, usize, State)> {
        if self.y < self.engine.height() {
            let x = self.x;
            let y = self.y;
            self.x += 1;
            if self.x >= self.engine.width {
                self.x = 0;
                self.y += 1;
            }
            Some((x, y, self.engine.get(x, y)))
        } else {
            None
        }
    }
}
