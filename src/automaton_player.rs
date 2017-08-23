use portaudio as pa;

use std::iter::FromIterator;
use std::sync::Arc;

use automaton::Automaton;
use cell::Cell;
use generation::Generation;
use rule::Rule;
use player::{Player, PlayerError};

pub struct AutomatonPlayer<C, R, G>
where
    C: Cell,
    R: Rule<G::NeighborData, C>,
    G: Generation<C> + IntoIterator<Item = C> + FromIterator<C>,
{
    automaton: Arc<Automaton<C, R, G>>,
    player: Player,
}

impl<C, R, G> AutomatonPlayer<C, R, G>
where
    C: Cell,
    R: Rule<G::NeighborData, C>,
    G: Generation<C> + IntoIterator<Item = C> + FromIterator<C>,
{
    pub fn new(automaton: Arc<Automaton<C, R, G>>, player: Player) -> Self {
        AutomatonPlayer {
            automaton: automaton,
            player: player,
        }
    }

    pub fn play(&self) -> Result<&Self, PlayerError> {
        self.player
            .play(move |in_buffer, out_buffer| {
            }).map(|_| self)
    }
}
