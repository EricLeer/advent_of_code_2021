use std::fs;

// Check 1, 4, 7 and 8 numbers (which have 2, 4, 3 and 7 panels respectivly)
// From this you know a, (c, f), (e, g), (b, d)
// from 0 and 8 you can deduce  d (the on where (b,d) is missing)
// from 6 and 8 you can deduce  c (the on where (c,f) is missing)
// from 9 and 8 you can deduce  e (the on where (e,g) is missing)
// Now all is known => Build up a sorted reprecentation of the numbers in a hasmap
//   For each input convert letters
//   check which numbers this belongs to
//   Save to hashmap where key is sorted original letters
// Loop over the output sort each digit and map to number
// Convert char numbers to int
// Add them together

fn process_input(puzzle_input: &str) -> usize{
    let mut result = 0;
    for line in puzzle_input.lines() {
        let split_line: Vec<&str> = line.split("|").collect();
        let input = split_line[0];
        let output = split_line[1];

        for digit in input.split_whitespace() {
            println!("{}", digit);
            break;
        }
    }
    result
}

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_8_a/data/test_input.txt";
    let puzzle_input = fs::read_to_string(filename).unwrap();

    // println!("{}", puzzle_input);
    let total_numbers = process_input(&puzzle_input);
    println!("total numbers: {}", total_numbers);
    
}
