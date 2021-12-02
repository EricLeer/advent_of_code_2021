use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

struct Submarine {
    horizontal: i32,
    depth: i32,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);

    let mut value_vector: Vec<String> = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap();
        value_vector.push(line);
    }
    value_vector
}

fn main() {
        // let filename = "/home/eric/projects/advent_of_code/day_2_a/data/test_input.txt";
        let filename = "/home/eric/projects/advent_of_code/day_2_a/data/puzzle_input.txt";

        let input_lines = lines_from_file(filename);

        let mut submarine = Submarine {
            horizontal: 0,
            depth: 0,
        };

        for line in input_lines {
            let mut values = line.split_whitespace();
            let command = values.next().unwrap_or("");
            let amount: i32 = values.next().unwrap_or("").parse().unwrap();
            match command {
                "forward" => submarine.horizontal += amount,
                "down" => submarine.depth += amount,
                "up" => submarine.depth -= amount, 
                _ => println!("Unkown command"),
            };
        }
        println!("postion: {}, depth: {}", submarine.horizontal, submarine.depth);

        let total_movement = submarine.horizontal * submarine.depth;

        println!("Total movement: {}", total_movement);
}
