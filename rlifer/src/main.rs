extern crate rlife_game;

use rlife_game::{Cell, Generation, Universe};

fn main() {
    let mut gen = Generation::new();
    gen.push(Cell{x: -1, y: 0});
    gen.push(Cell{x: 0, y: 0});
    gen.push(Cell{x: 1, y: 0});

    let u = Universe::new(gen);

    for g in u.take(3) {
        println!("g[{}] = {:?}", 0, g);
    }
}
