use std::fs;

// check for 2,3,4,7

fn process_input(puzzle_input: &str) -> usize{
    let mut result = 0;
    for line in puzzle_input.lines() {
        let output = line.split("|").collect::<Vec<&str>>()[1];
        for digit in output.split_whitespace() {
            let character_amount = digit.chars().count();
            println!("{}, {}", digit, character_amount);
            match character_amount {
                2 => {result += 1;},
                3 => {result += 1;},
                4 => {result += 1;},
                7 => {result += 1;},
                _ => (),
            }
        }
    }
    result
}

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_8_a/data/test_input.txt";
    let puzzle_input = fs::read_to_string(filename).unwrap();

    // println!("{}", puzzle_input);
    let easy_characters = process_input(&puzzle_input);
    println!("Amount of easy characters: {}", easy_characters);
    
}
