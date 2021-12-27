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

fn count_bits_from_input(input_lines: &Vec<String>) -> Vec<u32>{
    let line_length = input_lines[0].len();
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

fn get_bits(value: bool) -> (char, char) {
    let larger_bit;
    let smaller_bit;

    if value{
        larger_bit = '1';
        smaller_bit = '0';
    } else {
        larger_bit = '0';
        smaller_bit = '1';
    }
    (larger_bit, smaller_bit)
}

fn array_to_bits(bit_counts: Vec<u32>, total_bits: u32, gamma: bool) -> String {
    let mut result = String::new();
    
    let (larger_bit, smaller_bit) = get_bits(gamma);
    for bit in bit_counts.iter() {
        if bit > &(total_bits/2) {
            result.push(larger_bit);
        } else {
            result.push(smaller_bit);
        }
    }

    result
}

fn most_least_common(input_lines: &Vec<String>, index: usize, most_common: bool) -> char {
    // Check the most common bit type at position index over all input_lines
    let mut one_bit_count = 0;
    let total_bits = input_lines.len();

    for line in input_lines.iter() {
        if line.chars().nth(index).unwrap() == '1' {
            one_bit_count += 1;
        }
    };

    let (larger_bit, smaller_bit) = get_bits(most_common);

    if one_bit_count * 2 >= total_bits {
        return larger_bit
    } else {
        return smaller_bit
    }
}

fn filter_bit_array<'a>(bit_array: &Vec<String>, index: usize, most_common_bit: char) -> Vec<String> {
    // bit_array.iter().filter(|x| x.chars().nth(index).unwrap() == most_common_bit).collect()
    let mut result = vec![];
    for line in bit_array.iter() {
        if line.chars().nth(index).unwrap() == most_common_bit {
            result.push(line.clone());
        }
    }
    result
}

fn find_oxygen_c02(mut filtered_array: Vec<String>, oxygen: bool) -> isize {
    let mut bit_index = 0;
    while filtered_array.len() > 1 {
        let most_common_bit = most_least_common(&filtered_array, bit_index, oxygen);
        filtered_array = filter_bit_array(&filtered_array, bit_index, most_common_bit);

        bit_index += 1;
    }
    isize::from_str_radix(&(filtered_array[0]), 2).unwrap()

}

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_3_a/data/puzzle_input.txt";

    let input_lines = lines_from_file(filename);

    // Part A
    let total_bits = input_lines.len();
    let bit_counts = count_bits_from_input(&input_lines);

    let gamma_bits = array_to_bits(bit_counts.clone(), total_bits as u32, true); 
    let epsilon_bits = array_to_bits(bit_counts.clone(), total_bits as u32, false);

    let gamma = isize::from_str_radix(&gamma_bits, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_bits, 2).unwrap();
    println!("Result of part a: {}, {}, {}", gamma, epsilon, gamma * epsilon);

    // Part B

    let bit_oxygen = find_oxygen_c02(input_lines.clone(), true);
    let bit_co2 = find_oxygen_c02(input_lines.clone(), false);

    println!("Result of part b: {:?}, {:?}, {:?}", bit_oxygen, bit_co2, bit_oxygen*bit_co2);
}
