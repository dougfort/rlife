extern crate rlife_game;

use std::str::FromStr;

use rlife_game::{Cell, step};

fn main() {
 
    let c1 = Cell::from_str("(-1, 0)").unwrap();
    let c2 = Cell::from_str("(0, 0)").unwrap();
    let c3 = Cell::from_str("(1, 0)").unwrap();

    let mut generation = vec![];

    generation.push(c1);
    generation.push(c2);
    generation.push(c3);
    println!("g[{}] = {:?}", 0, generation);

    let generation = step(generation);
    println!("g[{}] = {:?}", 1, generation);
}
