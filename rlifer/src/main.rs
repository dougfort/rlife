extern crate rlife_game;

use std::str::FromStr;

use rlife_game::{Cell, Generation, Universe};

fn main() {
    let mut gen = Generation::new();

    let c1 = Cell::from_str("(-1,0)").unwrap();
    let c2 = Cell::from_str("(0,0)").unwrap();
    let c3 = Cell::from_str("(1,0)").unwrap();

    gen.push(c1);
    gen.push(c2);
    gen.push(c3);

    let u = Universe::new(gen);

    for g in u.take(3) {
        println!("g[{}] = {:?}", 0, g);
    }
}
