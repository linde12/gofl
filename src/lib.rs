use wasm_bindgen::prelude::*;

// pub struct Cell;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Cell {
    Alive = 1,
    Dead = 0,
}

#[wasm_bindgen(js_name = "Game")]
#[derive(Debug)]
pub struct Game {
    pub cols: usize,
    pub rows: usize,
    board: Vec<Cell>
}

#[wasm_bindgen(js_name = "Game")]
impl Game {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut board = Vec::with_capacity(cols * rows);
        for _ in 0..rows {
            for _ in 0..cols {
                board.push(Cell::Dead);
            }
        }

        Self { board, cols, rows }
    }

    pub fn insert(&mut self, col: usize, row: usize) {
        self.board[row * self.cols + col] = Cell::Alive;
    }

    pub fn get(&self, col: usize, row: usize) -> isize {
        match self.board.get(row * self.cols + col).unwrap_or(&Cell::Dead) {
            Cell::Alive => 1,
            Cell::Dead => 0,
        }
    }

    fn num_moore_neighbor(&self, pos: usize) -> u32 {
        // let mut count = 0;
        let map_x: isize = (pos % self.cols) as isize;
        let map_y: isize = ((pos - map_x as usize) / self.rows) as isize;

        println!("looking at {}, {} neighbors", map_x, map_y);

        // Count alive cells (x) around 'o', where o is `pos`
        // x x x
        // x o x
        // x x x
        // if an x is at a negative index, skip to next index
        let neighbors = vec![
            (map_x - 1, map_y - 1), (map_x, map_y - 1), (map_x + 1, map_y - 1),
            (map_x - 1, map_y),                         (map_x + 1, map_y),
            (map_x - 1, map_y + 1), (map_x, map_y + 1), (map_x + 1, map_y + 1),
        ];
        let count = neighbors.iter().fold(0, |acc, pos| {
            if pos.1 < 0 || pos.0 < 0 || pos.0 as usize >= self.cols || pos.1 as usize >= self.rows { return acc }

            if let Some(cell) = self.board.get(pos.1 as usize * self.cols + pos.0 as usize) {
                if &Cell::Alive == cell {
                    println!("found alive at {}", pos.1 as usize * self.cols + pos.0 as usize);
                    self.print();
                    println!("");
                    return acc + 1
                }
            }
            return acc
        });
        println!("count is {}", count);
        count
    }

    pub fn tick(&mut self) {
        self.board = self.board.clone().iter().enumerate().map(|(pos, cell)| {
            let num_neighbors = self.num_moore_neighbor(pos);
            match (cell, num_neighbors) {
                (Cell::Alive, 2..=3) => Cell::Alive, // Stayin' Alive
                (Cell::Dead, 3) => Cell::Alive, // Three makes life
                _ => Cell::Dead,
            }
        }).collect();
    }

    pub fn print(&self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                print!("{}", if self.get(x, y) == 1 { "*" } else { "." });
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
        let mut game = Game::new(3,3);
        game.insert(0, 1);
        game.insert(1, 1);
        game.insert(2, 1);
        // blinker horizontal
        assert_eq!(game.get(0,1), 1);
        assert_eq!(game.get(1,1), 1);
        assert_eq!(game.get(2,1), 1);
        println!("{:?}", game.board);
        game.tick();
        println!("{:?}", game.board);
        // blinker vertical
        assert_eq!(game.get(1,1), 1);
        assert_eq!(game.get(1,0), 1);
        assert_eq!(game.get(1,2), 1);
        // check so that horizontal 0 & 2 are dead
        assert_eq!(game.get(0,1), 0);
        assert_eq!(game.get(2,1), 0);
    }
}
