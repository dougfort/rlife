extern crate rlife_game;

use rlife_game::{Cell, Generation, step};

fn main() {
    let mut gen = Generation::new();
    gen.push(Cell{x: 0, y: 0});
    gen.push(Cell{x: 0, y: 1});
    gen.push(Cell{x: 0, y: 2});

    match step(gen) {
        Ok(next) =>println!("next = {:?}", next),
        Err(err) => println!("err: {:?}", err)
    }
}
