extern crate portaudio;
extern crate rand;
extern crate sample;

mod automaton;
mod automaton_player;
mod render;
mod cell;
mod generation;
mod player;
mod random_init;
mod rule;
mod rule90;
mod vector_generation;

use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;

use automaton::Automaton;
use automaton_player::AutomatonPlayer;
use cell::BasicCell;
use player::Player;
use random_init::RandomInit;
use render::Render;
use rule90::Rule90;
use vector_generation::VectorGeneration;

fn main() {
    let generation: VectorGeneration<BasicCell> = VectorGeneration::randomized(140);
    let mut automaton = Arc::new(Automaton::new(Rule90, generation));

    loop {
        sleep(Duration::from_millis(100));
        println!("{}", automaton.render());
        automaton.evolve();
    }
}
