use aoc_2025_07::{file_parser::FileParser, solve, solve_pt1, solve_pt2};

fn main() {
    let my_file = FileParser::new("data/input.txt");
    let (part_1, part_2) = solve(my_file.get_str());
    println!("Part 1: {}", solve_pt1(my_file.get_str()));
    println!("Part 2: {}", solve_pt2(my_file.get_str()));
}
