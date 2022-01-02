// Use a hashmap which maps x,y coordinates to the amount of overlaps
// For each line loop through all line coordinates and add to the hasmap
// Loop through the hashmap and count all values higher then 2
use std::fs;
use std::collections::HashMap;
use std::cmp::{
    max,
    min
};
// add points to vector
// Iterate through vector, add in between points to hasmap
// Parse through hashmap counting all positions with more then 2 overlaps

struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

impl Line {
    fn print_line(&self) {
        println!("start: {}, {} end: {}, {}", self.start_x, self.start_y, self.end_x, self.end_y);
    }

    fn is_diagonal(&self) -> bool {
        // self.print_line();
        let x_diff = max(self.start_x, self.end_x) - min(self.start_x, self.end_x);
        let y_diff = max(self.start_y, self.end_y) - min(self.start_y, self.end_y);
        // println!("{}, {}", x_diff, y_diff);
        x_diff == y_diff
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
        // line.print_line();
        lines.push(line);
        
    }
    lines
}

fn unroll_line(start: i32, end: i32, fixed: i32, x_fixed: bool, tracking_hash: &mut HashMap<(i32, i32), i32>) {
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

fn sort_points(start: i32, end: i32) -> (i32, i32) {
    let start_out;
    let end_out;
    if start > end {
        start_out = end;
        end_out = start;
    } else {
        start_out = start;
        end_out = end;
    }
    (start_out, end_out)
}

fn unroll_diagonal_line(line: &Line, tracking_hash: &mut HashMap<(i32, i32), i32>) {
    // make sure that all types of diagonal are supported: / \ both in up and down direction
    // First make sure we are always going up, then make sure we are always going to the right. 
    // Always up:
    // start x < end x
    // Always right:
    // start y < end y
    let diff = max(line.start_x, line.end_x) - min(line.start_x, line.end_x);

    let mut x_sign = 1;
    let mut y_sign = 1;
    if line.start_x > line.end_x {
        x_sign = -1;
    }
    if line.start_y > line.end_y {
        y_sign = -1;
    }
    
    // line.print_line();
    for i in 0..(diff + 1) {
        // println!("i: {}, x: {}, y: {}, xsign: {}, ysign: {}", i, line.start_x, line.start_y, x_sign, y_sign);
        let stat = tracking_hash.entry((line.start_x + (x_sign * i), line.start_y + (y_sign * i))).or_insert(0);
        *stat += 1;
    }
}


fn build_hashmap(lines: Vec<Line>) -> HashMap<(i32, i32), i32>{
    let mut point_hashmap = HashMap::new();
    for line in lines.iter() {
        if line.start_x == line.end_x {
            // loop over y
            unroll_line(line.start_y, line.end_y, line.start_x, true, &mut point_hashmap);
        } else if line.start_y == line.end_y {
            // loop over x
            unroll_line(line.start_x, line.end_x, line.start_y, false, &mut point_hashmap);
        } else if line.is_diagonal() {
            unroll_diagonal_line(line, &mut point_hashmap);
        } else {
            // Ignore this line
            continue
        }
    }
    // println!("{}", point_hashmap.len());
    point_hashmap
}

fn count_overlapping_points(point_hashmap: HashMap<(i32, i32), i32>) -> i32 {
    let mut overlap_count = 0;
    for (key, val) in point_hashmap.iter() {
        if val > &1 {
            // println!("{:?}", key);
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
