extern crate rand;

mod automaton;
mod render;
mod cell;
mod generation;
mod random_init;
mod rule;
mod rule90;
mod vector_generation;

use std::thread::sleep;
use std::time::{Duration};

use automaton::Automaton;
use cell::BasicCell;
use random_init::RandomInit;
use render::Render;
use rule90::Rule90;
use vector_generation::VectorGeneration;

fn main() {
    let generation: VectorGeneration<BasicCell> =
        VectorGeneration::randomized(80);
    
    let mut automaton = Automaton::new(
        Rule90,
        generation
    );

    loop {
        sleep(Duration::from_millis(100));
        println!("{}", automaton.render());
        automaton.evolve();        
    }
}
