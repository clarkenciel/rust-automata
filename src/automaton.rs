use std::iter::FromIterator;
use std::marker::PhantomData;

use cell::Cell;
use generation::Generation;
use rule::Rule;
use render::Render;

pub struct Automaton<C, R, G>
    where C: Cell,
          R: Rule<G::NeighborData,C>,
          G: Generation<C>
{
    generation: G,
    rule: R,
    phantom: PhantomData<C>
}

impl<C,R,G> Automaton<C, R, G>
    where C: Cell,
          R: Rule<G::NeighborData,C>,
          G: Generation<C> + IntoIterator<Item=C> + FromIterator<C>
{
    pub fn new(rule: R, generation: G) -> Self {
        Automaton { generation: generation, rule: rule, phantom: PhantomData }
    }

    pub fn evolve(&mut self) {
        self.generation = self.update();
    }

    fn update(&self) -> G {
        self.generation.neighbors().into_iter()
            .zip(self.generation.cells().into_iter())
            .map(|(ns, c)| self.rule.evaluate(&ns, &c))
            .collect()
    }
}

impl<C,R,G> Render for Automaton<C, R, G>
    where C: Cell,
          R: Rule<G::NeighborData,C>,
          G: Generation<C> + Render {
    fn render(&self) -> String {
        self.generation.render()
    }
}
