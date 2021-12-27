use std::fs;

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_4_a/data/test_input.txt";

    let puzzle_input = fs::read_to_string(filename).unwrap();

    let mut puzzle_input_iter = puzzle_input.lines();

    let bingo_numbers = puzzle_input_iter.next().unwrap();
    
    println!("{}", bingo_numbers);
}
