use std::ops::RangeBounds;

use serde::Deserialize;

const EMPTY: i8 = 0;

#[derive(Debug, Deserialize)]
pub struct Board {
    state: Vec<i8> // 0 means empty
}

impl Board {

    fn index_ok(row: usize, column: usize) -> bool {
        let rng = 1..=9;
        rng.contains(&row) && rng.contains(&column)
    }

    fn get(&self, row: usize, column: usize) -> Option<&i8> {
        if Self::index_ok(row, column) {
            self.state.get(row * 9 + column)
        }
        else {
            None
        }
    }

    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut i8> {
        if Self::index_ok(row, column) {
            self.state.get_mut(row * 9 + column)
        }
        else {
            None
        }
    }
}
