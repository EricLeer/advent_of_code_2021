use std::fs;

struct BingoNumber {
    number: u32,
    crossed: bool,
}

impl BingoNumber {
    fn new(number: u32) -> BingoNumber {
        BingoNumber { number: number, crossed: false }
    }
}

struct BingoBoard {
    board: Vec<BingoNumber>,
    won: bool,
}

impl BingoBoard {
    fn print_board(&self) -> () {
        for number in self.board.iter(){
            print!("{} ", number.number);
        }
    }

    fn cross_number(&mut self, number: u32) -> () {
        for bingo_number in self.board.iter_mut() {
            if bingo_number.number == number {
                bingo_number.crossed = true;
            }
        }
    }

    fn index_board(&self, row: usize, col: usize) -> &BingoNumber {
        &self.board[(row*5)+col]
    }

    fn check_win(&self) -> bool {
        let mut all_crossed = true;

        // Check rows
        for row in 0..5 {
            'inner_row: for col in 0..5 {
                let number = self.index_board(row, col);
                if number.crossed {
                    continue
                } else {
                    all_crossed = false;
                    break 'inner_row
                }
            }
            if all_crossed {
                return true
            } else {
                all_crossed = true;
            }
        }

        for col in 0..5 {
            'inner_col: for row in 0..5 {
                let number = self.index_board(row, col);
                if number.crossed {
                    continue
                } else {
                    all_crossed = false;
                    break 'inner_col
                }
            }
            if all_crossed {
                return true
            } else {
                all_crossed = true;
            }
        }
        return false
    }

    fn score(&self) -> u32 {
        let mut result = 0;
        for number in self.board.iter() {
            if !number.crossed {
                result += number.number
            }
        }
        result
    }
}

fn parse_board(bingo_numbers: &str) -> BingoBoard {
    let mut bingo_board = BingoBoard{ board: vec![], won: false};
    
    for line in bingo_numbers.lines() {
        for number in line.split_whitespace() {
            let bingo_number = BingoNumber::new(number.parse().unwrap());
            bingo_board.board.push(bingo_number);
        }
    }
    bingo_board
}

fn check_allboards_won(boards: &Vec<BingoBoard>) -> bool {
    for board in boards.iter() {
        if !board.won {
            return false
        }
    }
    return true
}

fn main() {
    let filename = "/Users/ericleer/projects/rust_projects/advent_of_code_2021/day_4_a/data/puzzle_input.txt";
    let puzzle_input = fs::read_to_string(filename).unwrap();
    let mut win = None;

    let puzzle_input: Vec<&str> = puzzle_input
        .trim()
        .split("\n\n")
        .collect();
    let numbers = &puzzle_input[0];
    let boards = &puzzle_input[1..];

    let mut boards: Vec<BingoBoard> = boards.iter().map(|x| parse_board(x)).collect();
    for number in numbers.split(",") {
        let number = number.parse().unwrap();
        for board in boards.iter_mut() {
            if board.won {
                continue;
            }
            board.cross_number(number);

            if board.check_win() {
                // check if last board
                board.won = true;
                win = Some(board.score() * number);
            }
        }

        if win.is_some() & check_allboards_won(&boards){
            break
        }

    }

    println!("{}", win.unwrap());
}
