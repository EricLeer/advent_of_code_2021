// Use a hashmap which maps x,y coordinates to the amount of overlaps
// For each line loop through all line coordinates and add to the hasmap
// Loop through the hashmap and count all values higher then 2
use std::fs;

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_5_a/data/test_input.txt";
    let puzzle_input = fs::read_to_string(filename).unwrap();
    
    for line in puzzle_input.lines() {
        let points: Vec<&str> = line.split("->").collect();
        let start_point: Vec<&str> = points[0].split(",").collect();
        let end_point: Vec<&str> = points[1].split(",").collect();
        
        let start_x = start_point[0];
        let start_y = start_point[1];

        let end_x = end_point[0];
        let end_y = end_point[1];
        println!("start: {}, {} end: {}, {}", start_x, start_y, end_x, end_y);
    }
}
