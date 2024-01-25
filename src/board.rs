use std::array;
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

    /* 
     * consumes an Iterator and checks 
     * whether a value appears only once in it 
     */
    fn appears_once<T: PartialEq>(it: impl Iterator<Item = T>, val: T) -> bool {
        it.filter(|x| *x == val).count() == 1
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
            }
            else {
                *val += 1;
                let val = self.get(srow, scol).unwrap();

                if Self::appears_once(self.get_box(srow, scol).iter(), val) && 
                   Self::appears_once(self.get_row(srow).iter(), val) && 
                   Self::appears_once(self.get_column(scol).iter(), val){

                       empty_index += 1;
                       if empty_index == empty_spots.len() {
                           return true;
                       }
                }
            }
        }
    }
}
