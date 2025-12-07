use std::cmp::Ordering;
use std::{collections::BTreeSet, io::Split};

// Y, THEN X
type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SplitterState {
    Ready,
    Energized,
    Spent,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    pub coord: Coord,
    state: SplitterState,
}

impl Node {
    fn new_splitter(coord: Coord) -> Self {
        Self {
            coord,
            state: SplitterState::Ready,
        }
    }

    fn new_starter(coord: Coord) -> Self {
        Self {
            coord,
            state: SplitterState::Energized,
        }
    }

    fn execute(&mut self) -> bool {
        if self.state == SplitterState::Energized {
            self.state = SplitterState::Spent;
            return true;
        }
        false
    }

    fn energize(&mut self) -> bool {
        if self.state == SplitterState::Ready {
            self.state = SplitterState::Energized;
            return true;
        }
        false
    }
}

pub struct Tachyon {
    grid: Vec<Vec<Option<Node>>>,
    cols_out: Vec<bool>,
    nodes_to_be_evaluated: Vec<Coord>,
    start: Coord,
    pub splits: u64,
}

impl Tachyon {
    pub fn new(input: &str) -> Self {
        let input_in_rows: Vec<&str> = input.lines().collect();
        let rows = input_in_rows.len();
        let cols = input_in_rows[0].len();
        let mut start: Option<Coord> = None;

        let mut new_grid: Vec<Vec<Option<Node>>> = vec![vec![None; cols]; rows];
        for row in 0..rows {
            let row_str = input_in_rows[row].as_bytes();
            for col in 0..cols {
                match row_str[col] as char {
                    'S' => {
                        new_grid[row][col] = Some(Node::new_starter((row, col)));
                        start = Some((row, col));
                    }
                    '^' => {
                        new_grid[row][col] = Some(Node::new_splitter((row, col)));
                    }
                    _ => {}
                }
            }
        }

        Self {
            grid: new_grid,
            cols_out: vec![false; cols],
            nodes_to_be_evaluated: Vec::with_capacity(rows * cols / 4),
            start: start.unwrap(),
            splits: 0,
        }
    }

    pub fn execute_round(&mut self) {
        self.fire(self.start);
        while !self.nodes_to_be_evaluated.is_empty() {
            let coord_vec_clone = self.nodes_to_be_evaluated.clone();
            self.nodes_to_be_evaluated.clear();
            for coord in coord_vec_clone {
                self.split(coord);
            }
        }
    }

    pub fn get_outputs(&self) -> u64 {
        let mut counter = 0u64;
        for col in &self.cols_out {
            if *col {
                counter += 1;
            }
        }
        counter
    }

    fn fire(&mut self, origin: Coord) {
        for row in (origin.0 + 1)..self.grid.len() {
            if let Some(detected_node) = &mut self.grid[row][origin.1] {
                if detected_node.energize() {
                    self.nodes_to_be_evaluated.insert(0, (row, origin.1));
                }
                return;
            }
        }
        self.cols_out[origin.1] = true;
    }

    fn split(&mut self, origin: Coord) {
        if !self.grid[origin.0][origin.1].unwrap().execute() {
            return;
        }

        self.splits += 1;

        self.fire((origin.0, origin.1 - 1));
        self.fire((origin.0, origin.1 + 1));
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn quick() {}
// }
