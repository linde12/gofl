use std::collections::{HashSet, HashMap};
use wasm_bindgen::prelude::*;

type Cell = (i32, i32);

#[wasm_bindgen(js_name = "Game")]
#[derive(Debug)]
pub struct Game {
    pub cols: usize,
    pub rows: usize,
    board: HashSet<Cell>
}

#[wasm_bindgen(js_name = "Game")]
impl Game {
    pub fn new(cols: usize, rows: usize) -> Self {
        let board = HashSet::new();
        Self { board, cols, rows }
    }

    pub fn insert(&mut self, col: i32, row: i32) {
        self.board.insert((col, row));
    }

    pub fn get(&self, col: i32, row: i32) -> bool {
        self.board.contains(&(col, row))
    }

    // Traverses all cells and updates the count for the neighbors
    fn neighbor_counts(&self, cells: &HashSet<Cell>) -> HashMap<Cell, i32> {
        let mut map = HashMap::new();
        for cell in cells.iter()
            .flat_map(|&(x, y)| {
                // Update the neighbor count for each neighboring cell (x)
                // This count signifies how many neighbors that cell has.
                // x x x
                // x o x
                // x x x
                vec![
                    (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                    (x - 1, y),                 (x + 1, y),
                    (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
                ]
            }) {
                *map.entry(cell).or_insert(0) += 1;
            }
        map
    }

    pub fn tick(&mut self) {
        self.board = self.neighbor_counts(&self.board).into_iter().filter_map(|(cell, num_neighbors)| {
            match (num_neighbors, self.board.contains(&cell)) {
                (2, true) | (3, ..) => Some(cell), // 2 && alive => OK, 3 => OK
                _ => None, // dead
            }
        }).collect();
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
        assert_eq!(game.get(0,1), true);
        assert_eq!(game.get(1,1), true);
        assert_eq!(game.get(2,1), true);
        println!("{:?}", game.board);
        game.tick();
        println!("{:?}", game.board);
        // blinker vertical
        assert_eq!(game.get(1,1), true);
        assert_eq!(game.get(1,0), true);
        assert_eq!(game.get(1,2), true);
        // check so that horizontal 0 & 2 are dead
        assert_eq!(game.get(0,1), false);
        assert_eq!(game.get(2,1), false);
    }
}
