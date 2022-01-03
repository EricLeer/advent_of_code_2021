use std::fs;
use std::cmp::{
    max,
    min
};

fn add_numbers(distance: usize) -> usize {
    let mut fuel = 0;
    for i in 0..(distance+1) {
        fuel += i;
    }
    fuel
}

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_7_a/data/puzzle_input.txt";
    let part_b = true;
    let puzzle_input = fs::read_to_string(filename).unwrap();

    println!("input: {}", puzzle_input);
    let numbers: Vec<usize> = puzzle_input.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    // let min_numb = numbers.into_iter().min().unwrap();
    let max_numb = numbers.clone().into_iter().max().unwrap();

    // let best_position = None;
    let mut least_distance = None;
    println!("min: {}, max: {}", 0, max_numb);
    for i in 0..max_numb {
        let mut total_distance = 0;
        for numb in numbers.iter() {
            let distance = max(numb, &i) - min(numb, &i);
            if part_b {
                total_distance += add_numbers(distance);
            } else {
                total_distance += distance;
            }
        }

        if least_distance.is_some() {
            if least_distance.unwrap() > total_distance {
                least_distance = Some(total_distance)
            }
        } else {
            least_distance = Some(total_distance)
        }

        println!("postion: {}, total_distance: {}", i, total_distance);
    }

    println!("Least fuel: {}", least_distance.unwrap());
    

}
