use std::str::FromStr;

use rlife_game::{step, Cell};

fn main() {
    let generation = vec![
        Cell::from_str("(-1, 0)").unwrap(),
        Cell::from_str("(0, 0)").unwrap(),
        Cell::from_str("(1, 0)").unwrap(),
    ];
    println!("g[0] = {:?}", generation);

    for i in 0..10 {
        let generation = step(&generation);
        println!("g[{}] = {:?}", i + 1, generation);
    }
}
