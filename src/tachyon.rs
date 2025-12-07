use std::cmp::Ordering;
use std::rc::Rc;
use std::{collections::BTreeSet, io::Split};

// Y, THEN X
type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SplitterState {
    Ready,
    Energized,
    Spent,
}

#[derive(Debug, Clone)]
struct Node<'a> {
    pub coord: Coord,
    state: SplitterState,
    paths_to_get_here: u64,
    parents: Vec<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn new_splitter(coord: Coord) -> Self {
        Self {
            coord,
            state: SplitterState::Ready,
            paths_to_get_here: 0,
            parents: Vec::new(),
        }
    }

    fn new_starter(coord: Coord) -> Self {
        Self {
            coord,
            state: SplitterState::Energized,
            paths_to_get_here: 1,
            parents: Vec::new(),
        }
    }

    fn new_collector(coord: Coord) -> Self {
        Self {
            coord,
            state: SplitterState::Spent,
            paths_to_get_here: 0,
            parents: Vec::new(),
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

    fn get_possible_paths_here(&self) -> u64 {
        self.paths_to_get_here
    }

    fn calculate_possible_paths_here(&mut self) {
        for parent in &self.parents {
            self.paths_to_get_here += parent.get_possible_paths_here();
        }
        self.parents.clear();
    }

    fn register_parent(&mut self, other: &'a Self) {
        self.parents.push(other);
    }
}

pub struct Tachyon<'a> {
    grid: Vec<Vec<Option<Node<'a>>>>,
    cols_out: Vec<bool>,
    nodes_to_be_evaluated: Vec<Coord>,
    start: Coord,
    pub splits: u64,
}

impl<'a> Tachyon<'a> {
    pub fn new(input: &str) -> Self {
        let input_in_rows: Vec<&str> = input.lines().collect();
        let rows = input_in_rows.len();
        let cols = input_in_rows[0].len();
        let mut start: Option<Coord> = None;

        let mut new_grid: Vec<Vec<Option<Node>>> = vec![vec![None; cols]; rows + 1];
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

        for col in 0..cols {
            new_grid[rows][col] = Some(Node::new_collector((rows, col)));
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
        self.fire(self.start, self.start);
        while !self.nodes_to_be_evaluated.is_empty() {
            let mut coord_vec_clone =
                Vec::<Coord>::with_capacity(self.nodes_to_be_evaluated.capacity());

            std::mem::swap(&mut coord_vec_clone, &mut self.nodes_to_be_evaluated);

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

    pub fn get_total_possible_paths(&mut self) -> u64 {
        let mut total = 0u64;
        let cols = self.grid[0].len();

        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                if let Some(node) = &mut self.grid[row][col] {
                    node.calculate_possible_paths_here();
                }
            }
        }

        if let Some(collectors) = self.grid.last() {
            let mut col_counter = 0u64;
            for collector in collectors {
                total += collector.as_ref().unwrap().get_possible_paths_here();
                col_counter += 1;
            }
        }
        total
    }

    fn fire(&mut self, parent_coord: Coord, origin: Coord) {
        let (upper, lower) = self.grid.split_at_mut(origin.0 + 1);

        for row in 0..lower.len() {
            if let Some(detected_node) = &mut lower[row][origin.1] {
                // Register Parent
                if parent_coord.0 != 0 {
                    let parent_ref = upper[parent_coord.0][parent_coord.1].as_mut().unwrap();
                    let unsafe_parent_ref: &'a mut Node =
                        unsafe { &mut *(parent_ref as *mut Node) };
                    detected_node.register_parent(unsafe_parent_ref);
                } else {
                    detected_node.paths_to_get_here = 1;
                }

                if detected_node.energize() {
                    self.nodes_to_be_evaluated
                        .push((row + origin.0 + 1, origin.1));
                }
                return;
            }
        }
        self.cols_out[origin.1] = true;
    }

    fn split(&mut self, origin: Coord) {
        if !self.grid[origin.0][origin.1].as_mut().unwrap().execute() {
            return;
        }

        self.splits += 1;

        self.fire(origin, (origin.0, origin.1 - 1));
        self.fire(origin, (origin.0, origin.1 + 1));
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn quick() {}
// }
