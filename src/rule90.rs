use cell::Cell;
use rule::Rule;

pub struct Rule90;

type NeighborData = (bool, bool);

impl<C> Rule<NeighborData, C> for Rule90 where C: Cell {
    fn evaluate(&self, neighbors: &NeighborData, cell: &C) -> C {
        match (*neighbors, cell.is_alive()) {
            // 010
            ((false, false), true) => Cell::dead(),
            // 000
            ((false, false), false) => Cell::dead(),
            // 110
            ((true, false), true) => Cell::alive(),
            // 111
            ((true, true), true) => Cell::dead(),
            // 001
            ((false, true), false) => Cell::alive(),
            // 011
            ((false, true), true) => Cell::alive(),
            // 101
            ((true, true), false) => Cell::dead(),
            // 100
            ((true, false), false) => Cell::alive()
        }
    }
}
