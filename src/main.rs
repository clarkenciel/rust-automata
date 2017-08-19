extern crate rand;

use std::thread::sleep;
use std::time::{Duration};
use rand::Rng;

fn main() {
    let mut automaton = Rule90::with_width(80);
    automaton.randomize();
    loop {
        sleep(Duration::from_millis(100));
        println!("{}", automaton.render());
        automaton.update();        
    }
}

trait Render {
    fn render(&self) -> String;
}

enum CellState {
    Alive,
    Dead
}

impl Render for CellState {
    fn render(&self) -> String {
        match self {
            &CellState::Alive => "●",
            &CellState::Dead => " "
        }.to_owned()
    }
}

trait Cell {
    fn is_alive(&self) -> bool;
    fn convert<B>(&self) -> B where B: FromCell {
        B::from_cell(self)
    }
}

trait FromCell {
    fn from_cell<T: ?Sized>(cell: &T) -> Self where T: Cell;
}

struct BasicCell {
    state: CellState,
}

impl BasicCell {
    fn dead() -> BasicCell {
        BasicCell { state: CellState::Dead }
    }

    fn alive() -> BasicCell {
        BasicCell { state: CellState::Alive }
    }
}

impl FromCell for BasicCell {
    fn from_cell<T: ?Sized>(cell: &T) -> Self where T: Cell {
        match cell.is_alive() {
            true => Self::alive(),
            false => Self::dead()
        }
    }
}

impl Render for BasicCell {
    fn render(&self) -> String {
        self.state.render().to_owned()
    }
}

impl Cell for BasicCell {    
    fn is_alive(&self) -> bool {
        match self.state {
            CellState::Alive => true,
            CellState::Dead => false
        }
    }
}

trait Automaton<C> where C: Cell {
    type Neighbors;

    fn generate_neighbors(pos: usize, cells: &Vec<C>) -> Self::Neighbors;
    fn rule(neighbors: &Self::Neighbors, cell: &C) -> C;
    fn cells(&self) -> &Vec<C>;
    fn update(&mut self);
    fn evolve(&self) -> Vec<C> {
        self.cells().iter().enumerate().map(|(pos, cell)| {
            let neighbors = Self::generate_neighbors(pos, self.cells());
            Self::rule(&neighbors, cell)
        }).collect()
    }
}

struct Rule90 {
    cells: Vec<BasicCell>
}

impl Rule90 {
    fn with_width(w: usize) -> Self {
        Rule90 {
            cells: (0..w).map(|i| {
                if i == w / 2 {
                    BasicCell::alive()
                } else {
                    BasicCell::dead()
                }
            }).collect()
        }
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for c in self.cells.iter_mut() {
            if rng.gen() { *c = BasicCell::alive() } else { *c = BasicCell::dead() };
        }
    }
}

impl Automaton<BasicCell> for Rule90 {
    type Neighbors = (bool, bool);

    fn update(&mut self) {
        self.cells = self.evolve();
    }

    fn generate_neighbors(pos: usize, cells: &Vec<BasicCell>) -> Self::Neighbors {
        let n1 = if pos <= 0 { false } else {
            cells.get(pos - 1)
                .map(BasicCell::from_cell)
                .map_or(false, |c| c.is_alive())
        };

        let n2 = if pos >= cells.len() - 1 { false } else {
            cells.get(pos + 1)
                .map(BasicCell::from_cell)
                .map_or(false, |c| c.is_alive())
        };
        (n1, n2)
    }

    fn rule(neighbors: &Self::Neighbors, cell: &BasicCell) -> BasicCell {
        match (*neighbors, cell.is_alive()) {
            // 010
            ((false, false), true) => BasicCell::dead(),
            // 000
            ((false, false), false) => BasicCell::dead(),
            // 110
            ((true, false), true) => BasicCell::alive(),
            // 111
            ((true, true), true) => BasicCell::dead(),
            // 001
            ((false, true), false) => BasicCell::alive(),
            // 011
            ((false, true), true) => BasicCell::alive(),
            // 101
            ((true, true), false) => BasicCell::dead(),
            // 100
            ((true, false), false) => BasicCell::alive()
        }
    }

    fn cells(&self) -> &Vec<BasicCell> {
        &self.cells
    }
}

impl Render for Rule90 {
    fn render(&self) -> String {
        self.cells.iter().fold(String::new(), |a, c| a + &c.render())
    }
}
