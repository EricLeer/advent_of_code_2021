use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

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

fn count_bits_from_input(input_lines: Vec<String>) -> Vec<u32>{
    let line_length = input_lines[0].len();
    println!("{}", line_length); 
    let mut result = vec![0; line_length];
    for line in input_lines.iter() {
        for (index, bit) in line.chars().enumerate() {
            if bit == '1' {
                result[index] += 1
            }
        }
    }
    result
}

fn array_to_bits(bit_counts: Vec<u32>, total_bits: u32, gamma: bool) -> String {
    let mut result = String::new();
    let larger_bit;
    let smaller_bit;
    if gamma{
        larger_bit = '1';
        smaller_bit = '0';
    } else {
        larger_bit = '0';
        smaller_bit = '1';
    }
    for bit in bit_counts.iter() {
        if bit > &(total_bits/2) {
            result.push(larger_bit);
        } else {
            result.push(smaller_bit);
        }
    }

    result
}

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_3_a/data/puzzle_input.txt";

    let input_lines = lines_from_file(filename);

    let total_bits = input_lines.len();
    let bit_counts = count_bits_from_input(input_lines);

    let gamma_bits = array_to_bits(bit_counts.clone(), total_bits as u32, true); 
    let epsilon_bits = array_to_bits(bit_counts.clone(), total_bits as u32, false);

    // println!("{:?}", bit_counts);

    // println!("{:?}", total_bits);

    // println!("{:?}", gamma_bits);

    let gamma = isize::from_str_radix(&gamma_bits, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_bits, 2).unwrap();
    println!("{}, {}, {}", gamma, epsilon, gamma * epsilon);
}
