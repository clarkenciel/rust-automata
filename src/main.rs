use std::thread::sleep;
use std::time::{Duration};

fn main() {
    let mut automaton = Automata::new(80, rule_30);
    for _ in 0..10_000 {
        sleep(Duration::from_millis(250));
        println!("{}", automaton.render());
        automaton = automaton.evolve();
    }
}

enum CellState {
    Alive,
    Dead
}

impl CellState {
    fn render(&self) -> &'static str {
        match self {
            &CellState::Alive => "●",
            &CellState::Dead => "○"
        }
    }
}

struct Cell {
    state: CellState,
}

impl Cell {
    fn dead() -> Cell {
        Cell { state: CellState::Dead }
    }

    fn alive() -> Cell {
        Cell { state: CellState::Alive }
    }
    
    fn render(&self) -> &'static str {
        self.state.render()
    }

    fn is_alive(&self) -> bool {
        match self.state {
            CellState::Alive => true,
            CellState::Dead => false
        }
    }
}

struct PositionedCell<'a> {
    cell: &'a Cell
}

impl<'a> PositionedCell<'a> {
    fn new(cell: &'a Cell) -> PositionedCell<'a> {
        PositionedCell { cell: cell }
    }
    
    fn is_alive(&self) -> bool {
        self.cell.is_alive()
    }
}

type Generation = Vec<Cell>;
type Neighbors<'a> = Vec<PositionedCell<'a>>;
type Rule = fn(&Neighbors, &Cell) -> Cell;

fn apply_rule(rule: &Rule, neighbors: &Neighbors, cell: &Cell) -> Cell {
    rule(neighbors, cell)
}

struct Automata {
    cells: Generation,
    rule: Rule
}

impl Automata {
    fn new(width: usize, rule: Rule) -> Self {
        let cells = (0..width).enumerate().map(|(i, _)| {
            if i == width / 2 {
                Cell::alive()
            } else {
                Cell::dead()
            }
        }).collect::<Generation>();
        
        Automata { cells: cells, rule: rule }
    }

    fn with_cells(&self, cells: Generation) -> Self {
        Automata { cells: cells, rule: self.rule }
    }
    
    fn evolve(&self) -> Self {
        self.with_cells(
            self.cells.iter().enumerate().map(|(i, cell)| {
                let left = if i >= 1 { i - 1 } else { 0 };
                let right = if i < self.cells.len() - 1 {
                    i + 1
                } else {
                    self.cells.len() - 1
                };
                
                let neighbors = vec![
                    PositionedCell::new(&self.cells[left]),
                    PositionedCell::new(&self.cells[right])
                ];
                
                apply_rule(&self.rule, &neighbors, &*cell)
            }).collect()
        )
    }

    fn render(&self) -> String {
        self.cells.iter().fold(String::new(), |output, cell| output + cell.render())
    }
}

fn rule_30(neighbors: &Neighbors, cell: &Cell) -> Cell {
    let cell_statuses = neighbors.iter()
        .map(|n| n.is_alive())
        .collect::<Vec<bool>>();
    
    match (cell_statuses, cell.is_alive()) {
        (ns, true) => {
            if ns[0] && ns[1] {
                Cell::dead()
            } else if ns[0] && !ns[1] {
                Cell::dead()
            } else if !ns[0] && ns[1] {
                Cell::alive()
            } else {
                Cell::alive()
            }
        },
        (ns, false) => {
            if ns[0] && ns[1] {
                Cell::dead()
            } else if ns[0] && !ns[1] {
                Cell::alive()
            } else if !ns[0] && ns[1] {
                Cell::alive()
            } else {
                Cell::dead()
            }
        }
    }
}
