use rand;
use rand::Rng;
use std::iter::{FromIterator,IntoIterator};
use std::vec::IntoIter;

use cell::Cell;
use generation::Generation;
use random_init::RandomInit;
use render::Render;

pub struct VectorGeneration<C: Cell> {
    cells: Vec<C>
}

impl<C> VectorGeneration<C> where C: Cell {
    pub fn new(width: usize) -> Self {
        VectorGeneration { cells: (0..width).map(|_| C::dead()).collect() }
    }
}

impl<C> RandomInit<C> for VectorGeneration<C> where C: Cell + Render {
    fn randomized(width: usize) -> Self {
        let mut rng = rand::thread_rng();
        VectorGeneration {
            cells: (0..width).map(|_| {
                if rng.gen() {
                    Cell::dead()
                } else {
                    Cell::alive()
                }
            }).collect()
        }                
    }
}

impl<C> IntoIterator for VectorGeneration<C> where C: Cell {
    type Item = C;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl<C> Render for VectorGeneration<C> where C: Cell + Render {
    fn render(&self) -> String {
        self.cells.iter().fold(String::new(), |s, c| s + &c.render())
    }
}

impl<C> FromIterator<C> for VectorGeneration<C> where C: Cell {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item=C> {
        VectorGeneration { cells: iter.into_iter().collect() }
    }
}

impl<C> Generation<C> for VectorGeneration<C> where C: Cell + Render {
    type NeighborData = (bool, bool);

    fn neighbors(&self) -> Vec<Self::NeighborData> {
        (0..self.cells.len()).into_iter().map(|i| {
            let prior = i.checked_sub(1).unwrap_or(0);
            let posterior = i.checked_add(1).unwrap_or(0);
            (
                self.cells.get(prior).map(|x| x.is_alive()).unwrap_or(false),
                self.cells.get(posterior).map(|y| y.is_alive()).unwrap_or(false)
            )
        }).collect()
    }
    
    fn cells(&self) -> &Vec<C> {
        &self.cells
    }
}
