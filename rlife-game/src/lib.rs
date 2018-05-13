use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cell {
    pub x: i32,
    pub y: i32
}

pub type Generation = Vec<Cell>;

pub struct Universe {
    current: Generation
}

impl Universe {
    pub fn new(gen: Generation) -> Universe {
        Universe{current: gen}
    }
}

#[derive(Debug)]
struct CellState {
    live : bool,
    count: u32
}

impl Iterator for Universe {
    type Item = Generation;

    fn next(&mut self) -> Option<Generation> {
        let mut neighbor_map: HashMap<Cell, CellState> = HashMap::new();

        for gen_cell in self.current.clone() {
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

        self.current = vec![];

         for (cell, state) in neighbor_map {    
            let lives = match state {
                CellState{live: true, count: 2} => true,
                CellState{live: true, count: 3} => true,
                CellState{live: false, count: 3} => true,
                _ => false
            }; 
            if lives {
                self.current.push(cell);
            }
        }
        Some(self.current.clone())
    }
}

