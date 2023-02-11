mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Live = 1,
}

#[wasm_bindgen]
pub struct Universe {
    cols: u32,
    rows: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(rows: u32, cols: u32) -> Self {
        let cells = (0..cols * rows)
            .map(|i| match i % 2 == 0 || i % 7 == 0 {
                true => Cell::Live,
                false => Cell::Dead,
            })
            .collect();

        Universe { cols, rows, cells }
    }

    pub fn cols(&self) -> u32 { self.cols }
    pub fn rows(&self) -> u32 { self.rows }
    pub fn cells_ptr(&self) -> *const Cell { self.cells.as_ptr() }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = self.get_index(row, col);
                let cell_state = self.cells[index];
                let cell_neighbors = self.count_neighbors(row, col);

                let next_cell = match (cell_state, cell_neighbors) {
                    (Cell::Live, x) if x < 2 => Cell::Dead,
                    (Cell::Live, 2) | (Cell::Live, 3) => Cell::Live,
                    (Cell::Live, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Live,
                    (otherwise, _) => otherwise,
                };

                next[index] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.cols + col) as usize
    }

    fn count_neighbors(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for d_row in [self.rows - 1, 0, 1].iter().cloned() {
            for d_col in [self.cols - 1, 0, 1].iter().cloned() {
                if d_row == 0 && d_col == 0 {
                    continue;
                }

                let n_row = (row + d_row) % self.rows;
                let n_col = (col + d_col) % self.cols;
                let idx = self.get_index(n_row, n_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}
