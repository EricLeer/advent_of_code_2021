use std::fs;

fn build_fish_array(puzzle_input: &String) -> Vec<usize>{
    let mut fish_array = vec![0; 9];
    for fish in puzzle_input.split(",").map(|x| x.parse::<usize>().unwrap()) {
        fish_array[fish] += 1;
    }
    fish_array
}

fn print_fish(fish_array: &Vec<usize>) {
    for fish in fish_array.iter() {
        print!("{}, ", fish);
    }
    println!(" ");
}

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_6_a/data/puzzle_input.txt";
    let timesteps = 256;

    let puzzle_input = fs::read_to_string(filename).unwrap();

    println!("{}", puzzle_input);
    let mut fish_array = build_fish_array(&puzzle_input);

    // print_fish(&fish_array);

    // let mut new_fish;
    for i in 0..timesteps {
        println!{"timestep: {}, amount of fish: {}", i, fish_array.iter().sum::<usize>()};
        print_fish(&fish_array);
        let breeding_fish = fish_array.remove(0);
        fish_array.push(breeding_fish);
        fish_array[6] += breeding_fish;

        // add_fish(&mut fish_array, new_fish);
    }

    println!("Total fish at end: {}", fish_array.iter().sum::<usize>());
}
