use std::iter::{FromIterator, IntoIterator};
use cell::Cell;
use render::Render;

pub trait Generation<C>: Render + FromIterator<C> + IntoIterator
where
    C: Cell,
{
    type NeighborData;

    fn neighbors(&self) -> Vec<Self::NeighborData>;
    fn cells(&self) -> &Vec<C>;
}
