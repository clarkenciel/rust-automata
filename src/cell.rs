use render::Render;

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

pub trait Cell {
    fn alive() -> Self;
    fn dead() -> Self;
    fn is_alive(&self) -> bool;
    fn convert<B>(&self) -> B where B: FromCell {
        B::from_cell(self)
    }
}

pub trait FromCell {
    fn from_cell<T: ?Sized>(cell: &T) -> Self where T: Cell;
}

pub struct BasicCell {
    state: CellState,
}

impl FromCell for BasicCell {
    fn from_cell<T: ?Sized>(cell: &T) -> Self where T: Cell {
        match cell.is_alive() {
            true => Self::alive(),
            false => Self::dead()
        }
    }
}

impl Cell for BasicCell {
    fn dead() -> BasicCell {
        BasicCell { state: CellState::Dead }
    }

    fn alive() -> BasicCell {
        BasicCell { state: CellState::Alive }
    }
    
    fn is_alive(&self) -> bool {
        match self.state {
            CellState::Alive => true,
            CellState::Dead => false
        }
    }
}

impl Render for BasicCell {
    fn render(&self) -> String {
        self.state.render().to_owned()
    }
}
