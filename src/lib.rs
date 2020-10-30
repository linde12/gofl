use std::collections::HashMap;

// pub struct Cell;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct Game {
    cols: usize,
    rows: usize,
    // TODO: Use HashSet in order to use less allocations
    board: HashMap<(usize, usize), Cell>,
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut board = HashMap::with_capacity(cols * rows);
        for y in 0..rows {
            for x in 0..cols {
                board.insert((x, y), Cell::Dead);
            }
        }

        Self { board, cols, rows }
    }

    pub fn insert(&mut self, col: usize, row: usize) {
        self.board.insert((col, row), Cell::Alive);
    }

    fn num_moore_neighbor(&self, pos: (usize, usize)) -> u32 {
        let mut count = 0;

        // Count alive cells (x) around 'o', where o is `pos`
        // x x x
        // x o x
        // x x x
        // if an x is at a negative index, skip to next index

        // TODO: refactor
        for x in -1..=1 {
            let nx = (pos.0 as isize) + x;
            if nx < 0 { continue; }
            for y in -1..=1 {
                if x == 0 && y == 0 { continue; }
                let ny = (pos.1 as isize) + y;
                if ny < 0 { continue; }
                if let Some(cell) = self.board.get(&(nx as usize, ny as usize)) {
                    if let Cell::Alive = cell {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn tick(&mut self) {
        self.board = self.board.iter().map(|(pos, cell)| {
            let num_neighbors = self.num_moore_neighbor(*pos);
            let new_state = match (cell, num_neighbors) {
                (Cell::Alive, 2..=3) => Cell::Alive, // Stayin' Alive
                (Cell::Dead, 3) => Cell::Alive, // Three makes life
                _ => Cell::Dead,
            };

            (*pos, new_state)
        }).collect();
    }

    // TODO: remove
    pub fn print(&self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                if let Some(cell) = self.board.get(&(x, y)) {
                    match cell {
                        Cell::Alive => print!("*"),
                        Cell::Dead => print!("."),
                    }
                }
            }
            println!("");
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut game = Game::new(20, 20);
        game.insert(0, 1);
        game.insert(1, 1);
        game.insert(2, 1);
        // blinker horizontal
        assert_eq!(game.board.get(&(0,1)), Some(&Cell::Alive));
        assert_eq!(game.board.get(&(1,1)), Some(&Cell::Alive));
        assert_eq!(game.board.get(&(2,1)), Some(&Cell::Alive));
        game.tick();
        // blinker vertical
        assert_eq!(game.board.get(&(1,1)), Some(&Cell::Alive));
        assert_eq!(game.board.get(&(1,0)), Some(&Cell::Alive));
        assert_eq!(game.board.get(&(1,2)), Some(&Cell::Alive));
        // check so that horizontal 0 & 2 are dead
        assert_eq!(game.board.get(&(0,1)), Some(&Cell::Dead));
        assert_eq!(game.board.get(&(2,1)), Some(&Cell::Dead));
    }
}
