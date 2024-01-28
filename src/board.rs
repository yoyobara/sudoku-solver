use std::{array, collections::HashSet, hash::Hash};
use serde::Deserialize;

const EMPTY: i8 = 0;

#[derive(Debug, Deserialize)]
pub struct Board {
    state: Vec<i8> // 0 means empty
}

impl Board {

    fn index_ok(row: usize, column: usize) -> bool {
        let rng = 0..=8;
        rng.contains(&row) && rng.contains(&column)
    }

    fn has_unique_non_empty<'a>(mut iter: impl Iterator<Item = &'a i8>) -> bool
    {
        let mut uniq = HashSet::new();
        iter.all(move |x| *x == 0 || uniq.insert(x))
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

    pub fn get_row(&self, row: usize) -> [i8; 9] {
        array::from_fn(|i| self.get(row, i).unwrap().to_owned())
    }

    pub fn get_column(&self, column: usize) -> [i8; 9] {
        array::from_fn(|i| self.get(i, column).unwrap().to_owned())
    }

    pub fn get_box(&self, row: usize, column: usize) -> [i8; 9] {
        let box_offset = (row / 3 * 3, column / 3 * 3);

        array::from_fn(|i| self.get(box_offset.0 + i / 3, box_offset.1 + i % 3).unwrap().to_owned())
    }

    fn get_empty_spots(&self) -> Vec<(usize, usize)> {
        let mut empty_spots: Vec<(usize, usize)> = Vec::new();

        for (i, cell) in self.state.iter().enumerate() {
           if *cell == EMPTY {
                empty_spots.push((i / 9, i % 9));
            }
        }

        empty_spots
    }

    pub fn solve(&mut self) -> bool {
        let empty_spots = self.get_empty_spots();
        let mut empty_index: usize = 0;

        loop {

            let (srow, scol) = empty_spots[empty_index];
            let val = self.get_mut(srow, scol).unwrap();

            if *val == 9 {
                *val = 0;
                if empty_index == 0 {
                    return false;
                }
                empty_index -= 1;
            }
            else {
                *val += 1;

                if Self::has_unique_non_empty(self.get_box(srow, scol).iter()) && 
                   Self::has_unique_non_empty(self.get_row(srow).iter()) && 
                   Self::has_unique_non_empty(self.get_column(scol).iter()){

                       empty_index += 1;
                       if empty_index == empty_spots.len() {
                           return true;
                       }
                }
            }
        }
    }
}
