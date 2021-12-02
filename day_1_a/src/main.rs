use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);

    let mut value_vector: Vec<i32> = Vec::new();
    for line in buf.lines() {
        // unwrap ignores possible errors that can happen during loading?
        let line = line.unwrap();

        let value = line.parse().unwrap();
        value_vector.push(value);
    }
    value_vector
}

fn is_larger(line_value: i32, previous_value: Option<i32>) -> i32 {
    if let Some(previous_sum) = previous_value {
        if line_value > previous_sum {
            return 1;
        }
        else {
            return 0;
        }
    }
    else {
        return 0;
    }
}

fn main() {
    // let filename = "/home/eric/projects/advent_of_code/day_1_a/data/test_case.txt";
    let filename = "/home/eric/projects/advent_of_code/day_1_a/data/puzzle_input.txt";
    println!("Opening file {}", filename);

    let lines = lines_from_file(filename);
    let mut result: Vec<i32> = Vec::new();

    let mut one_line_back: Option<i32> = None;
    let mut two_line_back: Option<i32> = None;
    let mut previous_line_sum: Option<i32> = None;

    let mut larger;

    for line in lines {
        match (one_line_back, two_line_back) {
            (Some(line_one), Some(line_two)) => {
                let line_sum = line + line_one + line_two;
                larger = is_larger(line_sum, previous_line_sum);
                previous_line_sum = Some(line_sum);
            },
            _ => larger = 0,
        }            

        
        println!("line: {}, Larger then previous: {}", line, larger);
        result.push(larger);

        two_line_back = one_line_back;
        one_line_back = Some(line);


    }

    let larger_count: i32 = result.iter().sum();

    println!("Amount of rows larger then the previous: {}", larger_count);
}
