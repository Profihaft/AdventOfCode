mod day02{
    pub mod day02;
}
mod day03{
    pub mod day03;
}
mod day04{
    pub mod day04;
}

use day02::day02::*;
use day03::day03::*;
use day04::day04::*;

fn main() {
    println!("--- Day 2 ---");
    day_two(r"src/day02/day02_input.txt").unwrap();
    println!("--- Day 3 ---");
    day_three(r"src\day03\day03_input.txt");
    println!("--- Day 4 ---");
    day_four(r"src\day04\day04_input.txt");
}