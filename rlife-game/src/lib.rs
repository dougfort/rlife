use std::io;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cell {
    pub x: i32,
    pub y: i32
}

pub type Generation = Vec<Cell>;

/// parse a string of the form "(x0, y0), (x1, y1)..."
pub fn parse(str: String) -> Result<Generation, io::Error> {


    return Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
}

#[derive(Debug)]
struct CellState {
    live : bool,
    count: u32
}

/// step the generation n into generation n+1
pub fn step(gen: Generation)-> Result<Generation, io::Error> {
    let mut neighbor_map: HashMap<Cell, CellState> = HashMap::new();

    for gen_cell in gen {
        for x in [(gen_cell.x-1), gen_cell.x, (gen_cell.x+1)].iter() {
            for y in [(gen_cell.y-1), gen_cell.y, (gen_cell.y+1)].iter() {
                let neighbor_cell = Cell{x: *x, y: *y};
                let ref mut state = neighbor_map.entry(neighbor_cell).or_insert(CellState{live: false, count:0});
                if neighbor_cell == gen_cell {
                    state.live = true;
                } else {
                    state.count += 1;
                }
            }
        }
    }

    let mut neighbor_vec: Vec<Cell> = vec![];

    for (cell, state) in neighbor_map {    
        println!("{:?} = {:?}", cell, state);   
        let lives = match state {
            CellState{live: true, count: 2} => true,
            CellState{live: true, count: 3} => true,
            CellState{live: false, count: 3} => true,
            _ => false
        }; 
        if lives {
            neighbor_vec.push(cell);
        }
    }
    Ok(neighbor_vec)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
