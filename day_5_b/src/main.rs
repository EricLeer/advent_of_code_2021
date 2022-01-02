// Use a hashmap which maps x,y coordinates to the amount of overlaps
// For each line loop through all line coordinates and add to the hasmap
// Loop through the hashmap and count all values higher then 2
use std::fs;
use std::collections::HashMap;

// add points to vector
// Iterate through vector, add in between points to hasmap
// Parse through hashmap counting all positions with more then 2 overlaps

struct Line {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
}

impl Line {
    fn print_line(&self) {
        println!("start: {}, {} end: {}, {}", self.start_x, self.start_y, self.end_x, self.end_y);
    }
}

fn parse_puzzle_input(input: String) -> Vec<Line> {
    let mut lines = vec![];
    for line in input.lines() {
        let points: Vec<&str> = line.split("->").collect();
        let start_point: Vec<&str> = points[0].split(",").collect();
        let end_point: Vec<&str> = points[1].split(",").collect();
        
        let line = Line {
            start_x: start_point[0].trim().parse().unwrap(),
            start_y: start_point[1].trim().parse().unwrap(),
            end_x: end_point[0].trim().parse().unwrap(),
            end_y: end_point[1].trim().parse().unwrap(),
        };
        line.print_line();
        lines.push(line);
        
    }
    lines
}

fn unroll_line(start: u32, end: u32, fixed: u32, x_fixed: bool, tracking_hash: &mut HashMap<(u32, u32), u32>) {
    let start_loop;
    let end_loop;

    if start > end {
       start_loop = end;
       end_loop = start; 
    } else {
        // Invert start of iteration
        start_loop = start;
        end_loop = end;
    }
    for i in start_loop..(end_loop + 1) {
        if x_fixed{
            // println!("{}, {}", fixed, i);
            let stat = tracking_hash.entry((fixed, i)).or_insert(0);
            *stat += 1;
        } else {
            // println!("{}, {}", i, fixed);
            let stat = tracking_hash.entry((i, fixed)).or_insert(0);
            *stat += 1;
        }
    }
}

fn build_hashmap(lines: Vec<Line>) -> HashMap<(u32, u32), u32>{
    let mut point_hashmap = HashMap::new();
    for line in lines.iter() {
        if line.start_x == line.end_x {
            // loop over y
            unroll_line(line.start_y, line.end_y, line.start_x, true, &mut point_hashmap);
        } else if line.start_y == line.end_y {
            // loop over x
            unroll_line(line.start_x, line.end_x, line.start_y, false, &mut point_hashmap);
        } else {
            // Ignore this line
            continue
        }
    }
    // println!("{}", point_hashmap.len());
    point_hashmap
}

fn count_overlapping_points(point_hashmap: HashMap<(u32, u32), u32>) -> u32 {
    let mut overlap_count = 0;
    for (key, val) in point_hashmap.iter() {
        if val > &1 {
            println!("{:?}", key);
            overlap_count += 1;
        }
    }
    overlap_count
}

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_5_b/data/puzzle_input.txt";
    let puzzle_input = fs::read_to_string(filename).unwrap();
    
    let lines = parse_puzzle_input(puzzle_input);

    let point_hashmap = build_hashmap(lines);

    let overlap_count = count_overlapping_points(point_hashmap);

    println!("Overlapping points: {}", overlap_count);
}
