use crate::day4::Box::Absent;
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub struct Storage {
    cols: usize,
    rows: usize,
    data: Vec<Vec<Box>>,
}

#[derive(Clone)]
pub enum Box {
    Present,
    Absent,
    ToRemove,
}

impl Storage {
    pub fn remove(&mut self, row: usize, col: usize) {
        let position = self
            .data
            .get_mut(row)
            .expect("wrong row")
            .get_mut(col)
            .expect("wrong col");
        *position = Box::ToRemove;
    }

    fn new_from_file(file: File) -> Result<Storage, Error> {
        let reader = BufReader::new(file);
        let mut data: Vec<Vec<Box>> = Vec::new();
        for line in reader.lines() {
            let string = line?;
            let mut array = Vec::with_capacity(string.len());
            for char in string.chars() {
                if char == '@' {
                    array.push(Box::Present)
                } else {
                    array.push(Box::Absent)
                }
            }
            data.push(array);
        }

        data.len();

        Ok(Storage {
            cols: data
                .get(0)
                .expect("there should be at least one line")
                .len(),
            rows: data.len(),
            data,
        })
    }
    pub fn surrounding(&self, row: usize, col: usize) -> usize {
        let mut result: usize = 0;

        let starting_row = row.checked_sub(1).unwrap_or(row);
        let starting_col = col.checked_sub(1).unwrap_or(col);

        for curr_row in starting_row..row + 2 {
            for curr_col in starting_col..col + 2 {
                if self.present(curr_row, curr_col) {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn present(&self, row: usize, col: usize) -> bool {
        let local_box = self.get_internal(row, col).unwrap_or(&Absent);
        match *local_box {
            Box::Present | Box::ToRemove => true,
            Box::Absent => false,
        }
    }
    fn get_internal(&self, row: usize, col: usize) -> Option<&Box> {
        let val = self.data.get(row)?.get(col)?;
        Some(val)
    }
    fn get_internal_mut(&mut self, row: usize, col: usize) -> Option<&mut Box> {
        let val = self.data.get_mut(row)?.get_mut(col)?;
        Some(val)
    }

    fn count_and_mark_surrounded(&mut self) -> usize {
        let mut result = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.present(col, row) {
                    if self.surrounding(col, row) < 5 {
                        self.remove(col, row);
                        result += 1;
                    }
                }
            }
        }
        result
    }

    fn clean(&mut self) -> bool {
        let mut result = false;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let current_box = self
                    .get_internal_mut(col, row)
                    .expect("Box should exist at this place");
                match current_box {
                    Box::ToRemove => {
                        *current_box = Absent;
                        result = true
                    }
                    _ => {}
                }
            }
        }
        result
    }
}

pub fn part1() -> Result<()> {
    let file = File::open("ressources/day4")?;

    let mut storage = Storage::new_from_file(file)?;
    let mut acc = 0;

    loop {
        acc += storage.count_and_mark_surrounded();
        if !storage.clean() {
            break;
        }
    }

    println!("result part 2 = {}", acc);
    // println!("result part 2 = {}", count_part2);
    Ok(())
}
