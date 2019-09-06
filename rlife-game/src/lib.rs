use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Cell {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let x_fromstr = coords[0].trim().parse::<i32>()?;
        let y_fromstr = coords[1].trim().parse::<i32>()?;

        Ok(Cell {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

#[derive(Debug)]
struct CellState {
    live: bool,
    count: u32,
}

pub fn step(current: &[Cell]) -> Vec<Cell> {
    let mut neighbor_map: HashMap<Cell, CellState> = HashMap::new();

    for gen_cell in current {
        for x in [(gen_cell.x - 1), gen_cell.x, (gen_cell.x + 1)].iter() {
            for y in [(gen_cell.y - 1), gen_cell.y, (gen_cell.y + 1)].iter() {
                let neighbor_cell = Cell { x: *x, y: *y };
                let state = &mut neighbor_map.entry(neighbor_cell).or_insert(CellState {
                    live: false,
                    count: 0,
                });
                if neighbor_cell == *gen_cell {
                    state.live = true;
                } else {
                    state.count += 1;
                }
            }
        }
    }

    let mut next = vec![];

    for (cell, state) in neighbor_map {
        let lives = match state {
            CellState {
                live: true,
                count: 2,
            } => true,
            CellState { count: 3, .. } => true,
            _ => false,
        };
        if lives {
            next.push(cell);
        }
    }

    next
}
