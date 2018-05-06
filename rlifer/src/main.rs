extern crate rlife_game;

use rlife_game::{Cell, Generation, step};

fn main() {
    let gen = Generation::new();
    gen.push(Cell{x: 0, y: 0});

    match step(gen) {
        Ok(next) => println!("ok"),
        Err(err) => println!("err: {:?}", err)
    }
}
