use cell::Cell;
use generation::Generation;
use render::Render;

pub trait RandomInit<C>: Generation<C>
where
    C: Cell + Render,
{
    fn randomized(width: usize) -> Self;
}
