use cell::Cell;

pub trait Rule<NeighborData, C: Cell> {
    fn evaluate(&self, neighbor_data: &NeighborData, cell: &C) -> C;
}
