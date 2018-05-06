use std::io;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub x: usize,
    pub y: usize
}

pub type Generation = Vec<Cell>;

/// parse a string of the form "(x0, y0), (x1, y1)..."
pub fn parse(str: String) -> Result<Generation, io::Error> {
    return Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
}

/// step the generation n into generation n+1
pub fn step(gen: Generation)-> Result<Generation, io::Error> {
    return Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
