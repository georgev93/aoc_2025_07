use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use std::thread;

pub mod file_parser;
use crate::file_parser::FileParser;

mod tachyon;
use crate::tachyon::Tachyon;

pub fn solve_pt1(input_file: &str) -> u64 {
    let mut tachyon = Tachyon::new(input_file);
    tachyon.execute_round();
    tachyon.splits
}

pub fn solve_pt2(input_file: &str) -> u64 {
    let mut tachyon = Tachyon::new(input_file);
    tachyon.execute_round();
    tachyon.get_total_possible_paths()
}

pub fn solve(input_file: &str) -> (u64, u64) {
    let mut tachyon = Tachyon::new(input_file);
    tachyon.execute_round();
    tachyon.get_outputs();
    (tachyon.splits, tachyon.get_total_possible_paths())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PT1: u64 = 21;
    const EXAMPLE_PT2: u64 = 40;
    const ACTUAL_PT1: u64 = 1619;
    const ACTUAL_PT2: u64 = 23607984027985;

    #[test]
    fn example() {
        let my_file = FileParser::new("data/example.txt");
        let (part_1, part_2) = solve(my_file.get_str());
        assert_eq!(part_1, EXAMPLE_PT1);
        assert_eq!(part_2, EXAMPLE_PT2);
    }

    #[test]
    fn example_pts() {
        let my_file = FileParser::new("data/example.txt");
        assert_eq!(solve_pt1(my_file.get_str()), EXAMPLE_PT1);
        assert_eq!(solve_pt2(my_file.get_str()), EXAMPLE_PT2);
    }

    #[test]
    fn actual() {
        let my_file = FileParser::new("data/input.txt");
        let (part_1, part_2) = solve(my_file.get_str());
        assert_eq!(part_1, ACTUAL_PT1);
        assert_eq!(part_2, ACTUAL_PT2);
    }

    #[test]
    fn actual_pts() {
        let my_file = FileParser::new("data/input.txt");
        assert_eq!(solve_pt1(my_file.get_str()), ACTUAL_PT1);
        assert_eq!(solve_pt2(my_file.get_str()), ACTUAL_PT2);
    }
}
